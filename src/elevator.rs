//Import source modules
use crate::person::Person;
use crate::people::People;

/** Elevator struct schema
 *
 * An elevator has the following properties
 * - floor_on (usize): The floor that the elevator is currently on
 * - moving_up (bool): If true, the elevator is moving up, else it is moving down
 * - stopped (bool): If true, the elevator is stopped, else it is moving
 * - people (Vec<Person>): A vector listing the people on the elevator
 * - energy_up (f64): Base energy spent per floor when empty & moving up
 * - energy_down (f64): Base energy spent per floor when empty & moving down
 * - energy_coef (f64): Multiplier for calculating energy spent while traveling with people
 */
pub struct Elevator {
    pub floor_on: usize,
    pub moving_up: bool,
    pub stopped: bool,
    pub people: Vec<Person>,
    energy_up: f64,
    energy_down: f64,
    energy_coef: f64
}

/** Elevator type implementation
 *
 * The following functions are implemented for the Elevator type,
 * and are callable via
 *
 * //Example
 * let my_elevator: Elevator = elevator::from(5.0_f64, 2.5_f64, 0.5_f64);
 * let is_leaving: bool = my_person.is_leaving(&mut rng);
 */
impl Elevator {
    /** Elevator constructor function
     *
     * Initialize an elevator given its energy values, those being
     * energy spent traveling up and down, as well as the energy
     * coefficient/multiplier for when people are on the elevator
     *
     * The floor_on, moving_up, and stopped attributes are initialized
     * to 0_i32, true, and true respectively.
     */
    pub fn from(energy_up: f64, energy_down: f64, energy_coef: f64) -> Elevator {
        Elevator {
            floor_on: 0_usize,
            moving_up: false,
            stopped: true,
            people: Vec::new(),
            energy_up: energy_up,
            energy_down: energy_down,
            energy_coef: energy_coef
        }
    }
    
    /** get_energy_spent function
     *
     * Calculate the energy spent while the elevator is moving.
     * Accept the number of people currently on the elevator.
     * Return the total energy spent moving one floor.
     */
    pub fn get_energy_spent(&mut self) -> f64 {
        let energy_spent = if self.stopped {
                0.0_f64
            } else if self.moving_up {
                self.energy_up + (self.energy_coef * (self.people.len() as f64))
            } else {
                self.energy_down + (self.energy_coef * (self.people.len() as f64))
            };
        energy_spent
    }

    /** update_floor function
     *
     * Update the floor the elevator is on.
     * Increment or decrement the floor_on usize based on whether
     * the elevator is stopped and/or moving up.
     */
    pub fn update_floor(&mut self) -> usize {
        //If the elevator is stopped, then return early
        if self.stopped {
            return self.floor_on;
        }

        //If the elevator is moving then update the floor the elevator is on
        self.floor_on = if self.moving_up {
            self.floor_on + 1_usize
        } else {
            self.floor_on - 1_usize
        };

        //Loop through the elevator's people and update their floor accordingly
        for pers in self.people.iter_mut() {
            pers.floor_on = self.floor_on;
        }

        //Return the floor the elevator is on
        self.floor_on
    }
    
    /** get_nearest_dest_floor function
     *
     * Check the elevator for people, if found then find the nearest
     * destination floor to the elevator's current floor among those
     * people.  Return a tuple with the floor and the distance to it.
     */
    pub fn get_nearest_dest_floor(&self) -> (usize, usize) {
        //Get the current floor the elevator is on
        let floor_index: usize = self.floor_on;

        //Get the destination floors from the elevator, if none then return
        let dest_floors: Vec<usize> = self.get_dest_floors();
        if dest_floors.len() == 0_usize {
            return (0_usize, 0_usize);
        }

        //Initialize variables to track the nearest destination floor
        //and the min distance between here and a destination floor
        let mut nearest_dest_floor: usize = 0_usize;
        let mut min_dest_floor_dist: usize = 0_usize;

        //Calculate the distance between each dest floor and the current floor
        for dest_floor_index in dest_floors.iter() {
            let dest_floor_dist: usize = if floor_index > *dest_floor_index {
                floor_index - dest_floor_index
            } else {
                dest_floor_index - floor_index
            };

            //Check whether this is less than the current minimum, or if no
            //minimum has been assigned yet (in which case it is 0_usize)
            if min_dest_floor_dist == 0_usize || dest_floor_dist < min_dest_floor_dist {
                min_dest_floor_dist = dest_floor_dist;
                nearest_dest_floor = *dest_floor_index;
            }
        }

        //Return the nearest destination floor
        (nearest_dest_floor, min_dest_floor_dist)
    }

    /** flush_people_leaving_elevator function
     *
     * Remove the people on the elevator whose destination
     * floor is the current floor and return a vector containing
     * those people.
     */
    pub fn flush_people_leaving_elevator(&mut self) -> Vec<Person> {
        //Initialize a vector of people for the people leaving
        let mut people_leaving: Vec<Person> = Vec::new();

        //Loop through the people on the elevator and add to the vec
        let mut removals = 0_usize;
        for i in 0..self.people.len() {
            //If the person is not on their destination floor, then skip
            if self.people[i-removals].floor_on != self.people[i-removals].floor_to {
                continue;
            }

            //If the person is on their destination floor, then remove them from
            //the elevator and add them to the leaving vec, incrementing the removals
            let mut person_leaving: Person = self.people.remove(i - removals);
            people_leaving.push(person_leaving);
            removals += 1_usize;
        }

        //Return the vector of people leaving
        people_leaving
    }
}

//Implement the extend trait for the elevator struct
impl Extend<Person> for Elevator {
    fn extend<T: IntoIterator<Item=Person>>(&mut self, iter: T) {
        for pers in iter {
            self.people.push(pers);
        }
    }
}

//Implement the people trait for the elevator struct
impl People for Elevator {
    /** get_dest_floors function
     *
     * Loop through the people on the elevator and calculate each
     * person's destination floor.  Return a vector of floor indices
     */
    fn get_dest_floors(&self) -> Vec<usize> {
        //Return the destination floors of the people on the elevator
        self.people.get_dest_floors()
    }

    /** are_people_going_to_floor funciton
     *
     * Determine whether there are people going to the given floor
     * Return a boolean representing this
     */
    fn are_people_going_to_floor(&self, floor_index: usize) -> bool {
        self.people.are_people_going_to_floor(floor_index)
    }
}