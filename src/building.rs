//Import external/standard modules
use std::fmt;
use rand::Rng;
use rand::distributions::{Distribution, Standard, Uniform, Bernoulli};
use crossterm::style::Stylize;

//Import source modules
use crate::person::Person;
use crate::people::People;
use crate::floor::Floor;
use crate::elevator::Elevator;

/** Building struct schema
 *
 * A Building has the following properties
 * - elevator (Elevator): An elevator for transporting people between floors
 * - floors (Vec<Floor>): A vector of floors composing the building
 * - avg_energy (f64): Average energy expendature by the building's elevator over time
 * - avg_wait_time (f64): Average wait time throughout the building per person waiting
 * - wait_time_denom (usize): The number of people whose wait time has been aggregated into the average
 * - dst_in (Bernoulli): The arrival probability distribution
 */
pub struct Building {
    pub elevator: Elevator,
    pub floors: Vec<Floor>,
    pub avg_energy: f64,
    pub avg_wait_time: f64,
    wait_time_denom: usize,
    dst_in: Bernoulli
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
    pub fn from(num_floors: usize, p_in: f64, energy_up: f64,
                energy_down: f64, energy_coef: f64) -> Building {
        //Initialize the Floors
        let mut floors: Vec<Floor> = {
            let mut tmp_floors: Vec<Floor> = Vec::new();
            for i in 0_usize..num_floors {
                let mut tmp_floor: Floor = Floor::new();
                tmp_floors.push(tmp_floor);
            }
            tmp_floors
        };
    
        //Initialize the Elevator
        let mut elevator: Elevator = Elevator::from(
            energy_up, energy_down, energy_coef
        );
    
        //Initialize the arrival probability distribution
        let dst_in = Bernoulli::new(p_in).unwrap();
    
        //Initialize and return the Building
        Building {
            floors: floors,
            elevator: elevator,
            avg_energy: 0_f64,
            avg_wait_time: 0_f64,
            wait_time_denom: 0_usize,
            dst_in: dst_in
        }
    }

    /** are_people_waiting_on_floor function
     *
     * Check the Nth floor for people waiting.  Return a boolean
     * representing whether people are waiting on that floor.
     */
    pub fn are_people_waiting_on_floor(&self, floor_index: usize) -> bool {
        self.floors[floor_index].are_people_waiting()
    }

    /** get_nearest_wait_floor function
     *
     * Check the building for people, and attempt to find the nearest
     * floor where people are waiting.  Return a tuple with the floor
     * and the distance to it.
     */
    pub fn get_nearest_wait_floor(&self) -> (usize, usize) {
        //Initialize variables to track the nearest waiting floor and
        //the min distance between here and that floor
        let mut nearest_wait_floor: usize = 0_usize;
        let mut min_wait_floor_dist: usize = 0_usize;

        //Loop through the floors and find the minimum distance floor
        //with waiting people
        for (i, floor) in self.floors.iter().enumerate() {
            //Check if there is anyone waiting on the floor, if not
            //then continue
            if !floor.are_people_waiting() {
                continue;
            }

            //Calculate the distance between this floor and the waiting
            //floor
            let wait_floor_dist: usize = if self.elevator.floor_on > i {
                self.elevator.floor_on - i
            } else {
                i - self.elevator.floor_on
            };

            //Check whether this is less than the current minimum, or
            //if no minimum has been assigned yet (in which case it is
            //0_usize)
            if min_wait_floor_dist == 0_usize || wait_floor_dist < min_wait_floor_dist {
                min_wait_floor_dist = wait_floor_dist;
                nearest_wait_floor = i;
            }
        }

        //Return the nearest waiting floor
        (nearest_wait_floor, min_wait_floor_dist)
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
        while self.dst_in.sample(&mut rng) {
            let mut new_person: Person = Person::from(0.05_f64, self.floors.len(), &mut rng);
            arrivals.push(new_person);
        }

        //Extend the first floor with the new arrivals
        self.floors[0].extend(arrivals);
    }

    /** gen_people_leaving function
     *
     * Given an RNG, generate people leaving based on their leaving
     * probability distribution.
     */
    pub fn gen_people_leaving(&mut self, mut rng: &mut impl Rng) {
        //Loop through the floors of the building
        for floor in self.floors.iter_mut() {
            //Generate the people leaving on that floor
            floor.gen_people_leaving(&mut rng);
        }
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
        //Get the current floor index and floor
        let floor_index: usize = self.elevator.floor_on;

        //Move people off the current floor
        let people_leaving_floor: Vec<Person> = self.floors[floor_index].flush_people_entering_elevator();
        let mut people_leaving_elevator: Vec<Person> = self.elevator.flush_people_leaving_elevator();

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
        self.elevator.extend(people_leaving_floor);
        self.floors[floor_index].extend(people_leaving_elevator);

        //If the current floor is the first floor, then flush the floor
        if floor_index == 0_usize {
            self.floors[floor_index].flush_people_leaving_floor();
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

    /** increment_wait_times function
     *
     * Increment the wait times for all people throughout the building
     */
    pub fn increment_wait_times(&mut self) {
        self.elevator.increment_wait_times();
        for floor in self.floors.iter_mut() {
            floor.increment_wait_times();
        }
    }
}

//Display trait implementation for a building
impl fmt::Display for Building {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut building_status: String = String::new();
        for (i, floor) in self.floors.iter().enumerate() {
            //Initialize strings representing this floor
            let mut floor_roof: String = String::from("|---|");
            let mut floor_body: String = format!("| {} |", floor.get_num_people());

            //If this floor has people waiting, then color it yellow
            if floor.are_people_waiting() {
                floor_roof = floor_roof.yellow().to_string();
                floor_body = floor_body.yellow().to_string();
            }

            //If this is the floor the elevator is on, then append the elevator as well
            if i == self.elevator.floor_on {
                //Initialize strings representing the elevator
                let elevator_roof: String = String::from("|---|");
                let elevator_body: String = format!("| {} |", self.elevator.get_num_people());

                //Append the elevator to the floor strings
                floor_roof.push_str(&elevator_roof);
                floor_body.push_str(&elevator_body);
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