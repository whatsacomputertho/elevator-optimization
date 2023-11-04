//Import external/standard modules
use std::fmt;
use rand::Rng;
use rand::distributions::{Distribution, Standard, Uniform, Bernoulli};

//Import source modules
use crate::person::Person;
use crate::floor::Floor;
use crate::elevator::Elevator;

/** Building struct schema
 *
 * A Building has the following properties
 * - elevator (Elevator): An elevator for transporting people between floors
 * - floors (Vec<Floor>): A vector of floors composing the building
 * - dst_in (Bernoulli): The arrival probability distribution
 */
pub struct Building {
    pub elevator: Elevator,
    pub floors: Vec<Floor>,
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
        //Loop until no new arrivals occur, for each arrival append a new person
        while self.dst_in.sample(&mut rng) {
            let mut new_person: Person = Person::from(0.05_f64, self.floors.len(), &mut rng);
            self.floors[0].people.push(new_person);
        }
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
     * elevator with the people who got on.
     */
    pub fn exchange_people_on_elevator(&mut self) {
        //Get the current floor index and floor
        let floor_index: usize = self.elevator.floor_on;

        //Move people off the current floor
        let people_leaving_floor: Vec<Person> = self.floors[floor_index].flush_people_entering_elevator();
        let people_leaving_elevator: Vec<Person> = self.elevator.flush_people_leaving_elevator();
        self.elevator.extend(people_leaving_floor);
        self.floors[floor_index].extend(people_leaving_elevator);

        //If the current floor is the first floor, then flush the floor
        if floor_index == 0_usize {
            self.floors[floor_index].flush_people_leaving_floor();
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
            let mut floor_body: String = format!("| {} |", floor.people.len());

            //If this is the floor the elevator is on, then append the elevator as well
            if i == self.elevator.floor_on {
                //Initialize strings representing the elevator
                let elevator_roof: String = String::from("|---|");
                let elevator_body: String = format!("| {} |", self.elevator.people.len());

                //Append the elevator to the floor strings
                floor_roof.push_str(&elevator_roof);
                floor_body.push_str(&elevator_body);
            }

            //Add the floor to the building status
            building_status = [floor_roof, floor_body, building_status].join("\n");
        }
        f.write_str(&building_status)
    }
}