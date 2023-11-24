//Import external/standard modules
use rand::Rng;

//Import source modules
use crate::person::Person;
use crate::people::People;

/** Floor struct schema
 *
 * A Floor has the following properties
 * - people (Vec<Person>): A vector of people currently on the floor
 * - dest_prob (f64): The probability that this floor is a destination
 */
pub struct Floor {
    people: Vec<Person>,
    pub dest_prob: f64
}

/** Floor type implementation
 *
 * The following functions are implemented for the Floor type,
 * and are callable via
 *
 * //Example
 * let my_floor: Floor = floor::new();
 * let num_people: usize = my_floor.get_num_people_waiting();
 */
impl Floor {
    /** Floor constructor function
     *
     * Initialize a new empty floor.
     */
    pub fn new() -> Floor {
        Floor {
            people: Vec::new(),
            dest_prob: 0_f64
        }
    }

    /** get_p_out function
     *
     * Calculate the probability that at least one person on this floor
     * will decide to leave next time step
     */
    pub fn get_p_out(&self) -> f64 {
        //If there is no one on the floor, return 0_f64
        if self.people.len() == 0 {
            return 0_f64;
        }

        //Initialize a p_out variable and a vec for each p_out
        let mut p_out: f64 = 0_f64;
        let mut past_p_outs: Vec<f64> = Vec::new();

        //Loop through the people in the floor and iteratively calculate
        //the p_out value
        for pers in self.people.iter() {
            //Calculate the product of each of the past people's inverse
            //p_out values
            let inverse_p_outs: f64 = {
                let mut tmp_inverse_p_outs: f64 = 1_f64;
                for past_p_out in &past_p_outs {
                    tmp_inverse_p_outs = tmp_inverse_p_outs * (1_f64 - past_p_out);
                }
                tmp_inverse_p_outs
            };

            //Calculate the summand value based on the person's p_out and
            //the product of each of the past people's p_out values
            let tmp_p_out: f64 = pers.p_out * inverse_p_outs;

            //Add the newly calculated value onto the p_out value and then
            //append the current p_out
            p_out += tmp_p_out;
            past_p_outs.push(pers.p_out);
        }

        //Return the p_out value
        p_out
    }

    /** gen_people_leaving function
     *
     * Generate the people on the floor who are leaving using
     * each person's gen_is_leaving function
     */
    pub fn gen_people_leaving(&mut self, rng: &mut impl Rng) {
        //Loop through the people on the floor and decide if they are leaving
        for pers in self.people.iter_mut() {
            //Skip people who are waiting for the elevator
            if pers.floor_on != pers.floor_to {
                continue;
            }

            //Randomly generate whether someone not waiting for the elevator will leave
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
            if self.people[i-removals].floor_on == self.people[i-removals].floor_to {
                continue;
            }

            //If the person is waiting, then remove them from the elevator
            //and add them to the leaving vec, incrementing the removals
            let person_entering_elevator: Person = self.people.remove(i - removals);
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
        self.people.retain_mut(|pers| if pers.is_leaving {
            false
        } else {
            true
        });
    }
}

//Implement the extend trait for the floor struct
impl Extend<Person> for Floor {
    fn extend<T: IntoIterator<Item=Person>>(&mut self, iter: T) {
        for pers in iter {
            self.people.push(pers);
        }
    }
}

//Implement the people trait for the floor struct
impl People for Floor {
    /** get_dest_floors function
     *
     * Call the people vec implementation of the function and return
     * the result.
     */
    fn get_dest_floors(&self) -> Vec<usize> {
        self.people.get_dest_floors()
    }

    /** get_num_people function
     *
     * Call the people vec implementation of the function and return
     * the result.
     */
    fn get_num_people(&self) -> usize {
        self.people.get_num_people()
    }

    /** get_num_people_waiting function
     *
     * Call the people vec implementation of the function and return
     * the result.
     */
    fn get_num_people_waiting(&self) -> usize {
        self.people.get_num_people_waiting()
    }

    /** get_aggregate_wait_time function
     *
     * Call the people vec implementation of the function and return
     * the result.
     */
    fn get_aggregate_wait_time(&self) -> usize {
        self.people.get_aggregate_wait_time()
    }

    /** are_people_going_to_floor funciton
     *
     * Call the people vec implementation of the function and return
     * the result.
     */
    fn are_people_going_to_floor(&self, floor_index: usize) -> bool {
        self.people.are_people_going_to_floor(floor_index)
    }

    /** are_people_waiting funciton
     *
     * Call the people vec implementation of the function and return
     * the result.
     */
    fn are_people_waiting(&self) -> bool {
        self.people.are_people_waiting()
    }

    /** increment_wait_times funciton
     *
     * Only increment the wait times of people who are waiting
     */
    fn increment_wait_times(&mut self) {
        //Loop through the people
        for pers in self.people.iter_mut() {
            //If the person is not waiting, then skip
            if pers.floor_on == pers.floor_to {
                continue;
            }

            //Increment the person's wait time if they are waiting
            pers.increment_wait_time();
        }
    }

    /** reset_wait_times funciton
     *
     * Only reset the wait times of people who are not waiting
     */
    fn reset_wait_times(&mut self) {
        //Loop through the people
        for pers in self.people.iter_mut() {
            //If the person is waiting, then skip
            if pers.floor_on != pers.floor_to {
                continue;
            }

            //Reset the person's wait time if they are not waiting
            pers.reset_wait_time();
        }
    }
}