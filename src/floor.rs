//Import external/standard modules
use std::fmt;
use rand::Rng;

//Import source modules
use crate::person::Person;

/** Floor struct schema
 *
 * A Floor has the following properties
 * - prop (typ): Description of the property
 */
 pub struct Floor {
    people: Vec<Person>
}

/** Floor constructor function
 *
 * Initialize a new empty floor.
 */
pub fn new() -> Floor {
    Floor {
        people: Vec::new()
    }
}

/** Floor type implementation
 *
 * The following functions are implemented for the Floor type,
 * and are callable via
 *
 * //Example
 * let my_floor: Floor = floor::new();
 * let is_leaving: bool = my_person.is_leaving(&mut rng);
 */
impl Floor {
    /** get_num_people function
     *
     * Calculate the number of people on the floor as a usize.
     * Return the usize tracking the number of people on the floor.
     */
    pub fn get_num_people(&mut self) -> usize {
        //Return the length of the people vector as a usize
        self.people.len() as usize
    }

    /** are_people_waiting function
     *
     * Check if there are any people waiting on the floor
     */
    pub fn are_people_waiting(&mut self) -> bool {
        //Initialize a bool to track if there are people waiting
        let mut is_person_waiting: bool = false;

        //Loop through the people on the floor and check if any are waiting
        for pers in self.people.iter_mut() {
            if !pers.is_waiting() {
                continue;
            }

            //Break if waiting person is found
            is_person_waiting = true;
            break;
        }

        //Return the boolean tracking if there is someone waiting
        is_person_waiting
    }

    /** get_num_people_waiting function
     *
     * Get the number of people waiting on the floor
     */
    pub fn get_num_people_waiting(&mut self) -> usize {
        //Initialize a usize to track the number of people waiting
        let mut num_people_waiting: usize = 0_usize;

        //Loop through the people on the floor and check if they are waiting
        for pers in self.people.iter_mut() {
            if !pers.is_waiting() {
                continue;
            }
            num_people_waiting += 1_usize;
        }

        //Return the number of people waiting
        num_people_waiting
    }

    /** gen_people_leaving function
     *
     * Generate the people on the floor who are leaving using
     * each person's gen_is_leaving function
     */
    pub fn gen_people_leaving(&mut self, mut rng: &mut impl Rng) {
        //Loop through the people on the floor and decide if they are leaving
        for pers in self.people.iter_mut() {
            let _is_person_leaving: bool = pers.gen_is_leaving(rng);
        }
    }

    /** flush_people_entering_elevator function
     *
     * Remove the people on the floor who are waiting for the
     * elevator. Return a vector containing those people.
     */
    pub fn flush_people_entering_elevator(&mut self) -> Vec<Person> {
        //Initialize a vector of people for the people entering the elevator
        let mut people_entering_elevator: Vec<Person> = Vec::new();

        //Loop through the people on the floor and add to the vec
        let mut removals = 0_usize;
        for i in 0..self.people.len() {
            //If the person is not waiting, then skip
            if !self.people[i-removals].is_waiting() {
                continue;
            }

            //If the person is waiting, then remove them from the elevator
            //and add them to the leaving vec, incrementing the removals
            let mut person_entering_elevator: Person = self.people.remove(i - removals);
            person_entering_elevator.set_on_elevator(true);
            people_entering_elevator.push(person_entering_elevator);
            removals += 1_usize;
        }

        //Return the vector of people leaving
        people_entering_elevator
    }

    /** flush_people_leaving_floor function
     *
     * Loop through the people on the floor and determine if anyone is leaving.
     * If so then remove them from the floor.
     *
     * This function presumably will only be executed when this is the first
     * floor.
     */
    pub fn flush_people_leaving_floor(&mut self) {
        //Loop through the floor and determine if anyone is leaving
        self.people.retain_mut(|pers| if pers.is_leaving() {
            println!("{} leaving building", pers);
            false
        } else {
            false
        });
    }
}

impl Extend<Person> for Floor {
    fn extend<T: IntoIterator<Item=Person>>(&mut self, iter: T) {
        for pers in iter {
            self.people.push(pers);
        }
    }
}