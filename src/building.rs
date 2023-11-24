//Import external/standard modules
use rand::Rng;
use rand::distributions::Distribution;
use statrs::distribution::Poisson;
use crossterm::style::Stylize;

//Import source modules
use crate::person::Person;
use crate::people::People;
use crate::floor::Floor;
use crate::floors::Floors;
use crate::elevator::Elevator;
use crate::elevators::Elevators;

//Constant representing the probability a person leaves the building during a time step
const P_OUT: f64 = 0.05_f64;

/** Building struct schema
 *
 * A Building has the following properties
 * - elevators (Vec<Elevator>): A vector of elevators for transporting people between floors
 * - floors (Vec<Floor>): A vector of floors composing the building
 * - avg_energy (f64): Average energy expendature by the building's elevator over time
 * - avg_wait_time (f64): Average wait time throughout the building per person waiting
 * - wait_time_denom (usize): The number of people whose wait time has been aggregated into the average
 * - p_in (f64): The lambda value for the arrival probability distribution
 * - dst_in (Poisson): The arrival probability distribution
 */
pub struct Building {
    pub elevators: Vec<Elevator>,
    pub floors: Vec<Floor>,
    pub avg_energy: f64,
    pub avg_wait_time: f64,
    wait_time_denom: usize,
    p_in: f64,
    dst_in: Poisson
}

/** Building type implementation
 *
 * The following functions are implemented for the Floor type,
 * and are callable via
 *
 * //Example
 * let my_building: Building = Building::new(...);
 * let are_people_waiting: bool = my_building.are_people_waiting_on_floor(0_usize);
 */
impl Building {
    /** Building constructor function
     *
     * Construct a building given the number of floors
     * it should have, its arrival probability, and its
     * Elevator parameters
     */
    pub fn from(num_floors: usize, num_elevators: usize, p_in: f64, energy_up: f64,
                energy_down: f64, energy_coef: f64) -> Building {
        //Initialize the Floors
        let floors: Vec<Floor> = {
            let mut tmp_floors: Vec<Floor> = Vec::new();
            for _ in 0_usize..num_floors {
                let tmp_floor: Floor = Floor::new();
                tmp_floors.push(tmp_floor);
            }
            tmp_floors
        };
    
        //Initialize the Elevators
        let elevators: Vec<Elevator> = {
            let mut tmp_elevators: Vec<Elevator> = Vec::new();
            for _ in 0_usize..num_elevators {
                let tmp_elevator: Elevator = Elevator::from(
                    energy_up, energy_down, energy_coef
                );
                tmp_elevators.push(tmp_elevator);
            }
            tmp_elevators
        };
    
        //Initialize the arrival probability distribution
        let dst_in = Poisson::new(p_in).unwrap();
    
        //Initialize and return the Building
        Building {
            floors: floors,
            elevators: elevators,
            avg_energy: 0_f64,
            avg_wait_time: 0_f64,
            wait_time_denom: 0_usize,
            p_in: p_in,
            dst_in: dst_in
        }
    }

    /** update_dest_probabilities function
     *
     * Loop through each floor and calculate the probability that
     * that floor becomes a waiting floor next time step.  Then set
     * the dest_prob attribute for each floor with the value.
     */
    pub fn update_dest_probabilities(&mut self) {
        //Get the number of floors in the building
        let num_floors: usize = self.floors.len() as usize;

        //Get the destination floors across each elevator
        let dest_floors: Vec<usize> = self.elevators.get_dest_floors();

        //Loop through the floors
        for (i, floor) in self.floors.iter_mut().enumerate() {
            //Initialize an f64 for this floor's probability
            let dest_probability: f64 = if i == 0 {
                //If this is the first floor, then calculate the prob
                //based on arrival probability only
                let people_waiting: f64 = {
                    let waiting: f64 = if floor.are_people_waiting() { 1_f64 } else { 0_f64 };
                    let going: f64 = if dest_floors.contains(&i) { 1_f64 } else { 0_f64 };
                    if waiting > going { waiting } else { going }
                };
                let p_in: f64 = self.p_in * ((num_floors as f64 - 1_f64)/(num_floors as f64));
                if people_waiting > p_in { people_waiting } else { p_in }
            } else {
                //If this is not the first floor, then calculate the
                //prob based on the elevator's people and the floor's
                //people and append it to the list
                let people_waiting: f64 = {
                    let waiting: f64 = if floor.are_people_waiting() { 1_f64 } else { 0_f64 };
                    let going: f64 = if dest_floors.contains(&i) { 1_f64 } else { 0_f64 };
                    if waiting > going { waiting } else { going }
                };
                let p_out: f64 = floor.get_p_out();
                if people_waiting > p_out { people_waiting } else { p_out }
            };
            floor.dest_prob = dest_probability;
        }
    }

    /** gen_people_arriving function
     *
     * Given an RNG, generate new people based on the arrival
     * probability distribution.  Add the new people to the first
     * floor.
     */
    pub fn gen_people_arriving(&mut self, mut rng: &mut impl Rng) {
        //Initialize a vector of Persons
        let mut arrivals: Vec<Person> = Vec::new();

        //Loop until no new arrivals occur, for each arrival append a new person
        for _ in 0_i32..self.dst_in.sample(&mut rng) as i32 {
            let new_person: Person = Person::from(P_OUT, self.floors.len(), &mut rng);
            arrivals.push(new_person);
        }

        //Extend the first floor with the new arrivals
        self.floors[0].extend(arrivals);
    }

    /** exchange_people_on_elevator function
     *
     * This function flushes the floor of its people waiting for the
     * elevator, and flushes the elevator of its people waiting to get
     * off.  It extends the floor with the people who got off, and the
     * elevator with the people who got on.  It also aggregates the
     * averages 
     */
    pub fn exchange_people_on_elevator(&mut self) {
        for elevator in self.elevators.iter_mut() {
            //If the elevator is not stopped then continue
            if !elevator.stopped {
                continue;
            }

            //Get the elevator's floor index
            let floor_index: usize = elevator.floor_on;

            //Move people off the floor and off the elevator
            let people_leaving_floor: Vec<Person> = self.floors[floor_index].flush_people_entering_elevator();
            let mut people_leaving_elevator: Vec<Person> = elevator.flush_people_leaving_elevator();

            //Aggregate the wait times of the people leaving the elevator into the average and reset
            let wait_times: usize = people_leaving_elevator.get_aggregate_wait_time();
            let num_people: usize = people_leaving_elevator.get_num_people();
            self.avg_wait_time = {
                let tmp_num: f64 = wait_times as f64 + (self.avg_wait_time * self.wait_time_denom as f64);
                let tmp_denom: f64 = num_people as f64 + self.wait_time_denom as f64;
                if tmp_denom == 0_f64 {
                    0_f64 //If the denominator is 0, return 0 to avoid NaNs
                } else {
                    tmp_num / tmp_denom
                }
            };
            self.wait_time_denom += num_people;
            people_leaving_elevator.reset_wait_times();

            //Extend the current floor and elevator with the people getting on and off
            elevator.extend(people_leaving_floor);
            self.floors[floor_index].extend(people_leaving_elevator);
        }
    }

    /** update_average_energy function
     *
     * Update the average energy expendature of the elevator given the
     * current time step.
     */
    pub fn update_average_energy(&mut self, time_step: i32, energy_spent: f64) {
        self.avg_energy = {
            let tmp_num: f64 = (self.avg_energy * time_step as f64) + energy_spent;
            let tmp_denom: f64 = (time_step + 1_i32) as f64;
            tmp_num / tmp_denom
        };
    }
}

//Display trait implementation for a building
impl std::fmt::Display for Building {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut building_status: String = String::new();
        let elevator_space: String = String::from("   \t ");
        for (i, floor) in self.floors.iter().enumerate() {
            //Initialize strings representing this floor
            let mut floor_roof: String = String::from("----\t||---\t||");
            let mut floor_body: String = format!("{:.2}\t||{}\t||", floor.dest_prob, floor.get_num_people());

            //If this floor has people waiting, then color it yellow
            if floor.are_people_waiting() {
                floor_roof = floor_roof.yellow().to_string();
                floor_body = floor_body.yellow().to_string();
            }

            //Loop through the elevators to check if any are on this floor
            let mut last_elevator_on_floor: usize = 0_usize;
            for (j, elevator) in self.elevators.iter().enumerate() {
                if elevator.floor_on != i as usize {
                    continue;
                }

                //If the elevator is on this floor, then display it i spaces away from the building
                let elevator_roof: String = format!("{}{}", str::repeat(&elevator_space, j - last_elevator_on_floor as usize), String::from("|-\t|"));
                let elevator_body: String = format!("{}|{}\t|", str::repeat(&elevator_space, j - last_elevator_on_floor as usize), elevator.get_num_people());

                //Append the elevator to the floor strings
                floor_roof.push_str(&elevator_roof);
                floor_body.push_str(&elevator_body);

                //Increment the counter for num elevators on this floor
                last_elevator_on_floor = j + 1_usize;
            }

            //Add the floor to the building status
            building_status = [floor_roof, floor_body, building_status].join("\n");
        }
        //Add the average energy and wait times throughout the building
        let wait_time_str: String = format!("Average wait time:\t{:.2}", self.avg_wait_time);
        let energy_str: String = format!("Average energy spent:\t{:.2}", self.avg_energy);
        building_status = [building_status, wait_time_str, energy_str].join("\n");

        //Format the string and return
        f.write_str(&building_status)
    }
}

//Floors trait implementation for a building
impl Floors for Building {
    /** are_people_waiting_on_floor function
     *
     * Call the floor vec implementation of the function and return
     * the result.
     */
    fn are_people_waiting_on_floor(&self, floor_index: usize) -> bool {
        self.floors.are_people_waiting_on_floor(floor_index)
    }

    /** get_nearest_wait_floor function
     *
     * Call the floor vec implementation of the function and return
     * the result.
     */
    fn get_nearest_wait_floor(&self, floor_on: usize) -> (usize, usize) {
        self.floors.get_nearest_wait_floor(floor_on)
    }

    /** get_dest_probabilities function
     *
     * Call the floor vec implementation of the function and return
     * the result.
     */
    fn get_dest_probabilities(&self) -> Vec<f64> {
        self.floors.get_dest_probabilities()
    }

    /** gen_people_leaving function
     *
     * Call the floor vec implementation of the function and return
     * the result.
     */
    fn gen_people_leaving(&mut self, rng: &mut impl Rng) {
        self.floors.gen_people_leaving(rng)
    }

    /** flush_first_floor function
     *
     * Call the floor vec implementation of the function and return
     * the result.
     */
    fn flush_first_floor(&mut self) {
        self.floors.flush_first_floor();
    }

    /** increment_wait_times function
     *
     * Call the floor vec and elevator vec implementations of the
     * function and return the result.
     */
    fn increment_wait_times(&mut self) {
        self.elevators.increment_wait_times();
        self.floors.increment_wait_times();
    }
}