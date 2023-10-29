//Import source modules
use crate::person::Person;

/** Elevator struct schema
 *
 * An elevator has the following properties
 * - people (Vec<Person>): A vector listing the people on the elevator
 * - energy_up (f64): Base energy spent per floor when empty & moving up
 * - energy_down (f64): Base energy spent per floor when empty & moving down
 * - energy_coef (f64): Multiplier for calculating energy spent while traveling with people
 * - floor_on (usize): The floor that the elevator is currently on
 * - moving_up (bool): If true, the elevator is moving up, else it is moving down
 * - stopped (bool): If true, the elevator is stopped, else it is moving
 */
pub struct Elevator {
    people: Vec<Person>,
    energy_up: f64,
    energy_down: f64,
    energy_coef: f64,
    floor_on: usize,
    moving_up: bool,
    stopped: bool
}

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
        people: Vec::new(),
        energy_up: energy_up,
        energy_down: energy_down,
        energy_coef: energy_coef,
        floor_on: 0_usize,
        moving_up: false,
        stopped: true
    }
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

    /** get_floor_on function
     *
     * Determine the floor the elevator is on.
     * Return the floor_on usize.
     */
    pub fn get_floor_on(&mut self) -> usize {
        self.floor_on
    }

    /** is_stopped function
     *
     * Determine whether the elevator is stopped.
     * Return the stopped boolean.
     */
    pub fn is_stopped(&mut self) -> bool {
        self.stopped
    }

    /** is_moving_up function
     *
     * Determine whether the elevator is moving up.
     * Return the moving_up boolean.
     */
    pub fn is_moving_up(&mut self) -> bool {
        self.moving_up
    }

    /** get_num_people function
     *
     * Calculate the number of people on the floor and also on
     * the elevator as a usize.
     */
     pub fn get_num_people(&mut self) -> usize {
        //Return the length of the people vector as a usize 
        self.people.len() as usize
    }

    /** are_people_going_to_floor funciton
     *
     * Determine whether there are people going to the given floor
     * Return a boolean representing this
     */
    pub fn are_people_going_to_floor(&mut self, floor_index: usize) -> bool {
        //Initialize a boolean tracking if people are going to the given floor
        let mut is_going_to_floor: bool = false;

        //Loop through the people on the elevator and check
        for pers in self.people.iter_mut() {
            //If the person is not going to the given floor then skip
            if pers.get_floor_to() != floor_index {
                continue;
            }

            //Otherwise update the boolean and break
            is_going_to_floor = true;
            break;
        }

        //Return the is_going_to_floor boolean
        is_going_to_floor
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
            pers.set_floor_on(self.floor_on);
        }

        //Return the floor the elevator is on
        self.floor_on
    }

    /** set_stopped function
     *
     * Set whether the elevator is stopped.
     * Update the stopped bool with the input bool.
     */
    pub fn set_stopped(&mut self, is_stopped: bool) {
        self.stopped = is_stopped;
    }

    /** set_moving_up function
     *
     * Set whether the elevator is moving up.
     * Update the moving_up bool with the input bool.
     */
    pub fn set_moving_up(&mut self, is_moving_up: bool) {
        self.moving_up = is_moving_up;
    }

    /** get_dest_floors function
     *
     * Loop through the people on the elevator and calculate each
     * person's destination floor.  Return a vector of floor indices
     */
    pub fn get_dest_floors(&mut self) -> Vec<usize> {
        //Initialize a vector of usizes for the destination floors
        let mut dest_floors: Vec<usize> = Vec::new();

        //Loop through the people on the elevator and determine their dest floors
        for pers in self.people.iter_mut() {
            let dest_floor: usize = pers.get_floor_to();
            dest_floors.push(dest_floor);
        }

        //Return the vector of destination floors
        dest_floors
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
            if self.people[i-removals].get_floor_on() != self.people[i-removals].get_floor_to() {
                continue;
            }

            //If the person is on their destination floor, then remove them from
            //the elevator and add them to the leaving vec, incrementing the removals
            let mut person_leaving: Person = self.people.remove(i - removals);
            person_leaving.set_on_elevator(false);
            people_leaving.push(person_leaving);
            removals += 1_usize;
        }

        //Return the vector of people leaving
        people_leaving
    }
}

impl Extend<Person> for Elevator {
    fn extend<T: IntoIterator<Item=Person>>(&mut self, iter: T) {
        for pers in iter {
            self.people.push(pers);
        }
    }
}