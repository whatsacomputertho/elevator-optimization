//Import source modules
use crate::person::Person;

//Define people trait
pub trait People {
    fn get_dest_floors(&self) -> Vec<usize>;

    fn get_num_people(&self) -> usize;

    fn get_num_people_waiting(&self) -> usize;

    fn get_aggregate_wait_time(&self) -> usize;

    fn are_people_going_to_floor(&self, floor_index: usize) -> bool;

    fn are_people_waiting(&self) -> bool;

    fn increment_wait_times(&mut self);

    fn reset_wait_times(&mut self);
}

//Implement people trait for Vec<Person>
impl People for Vec<Person> {
    /** get_dest_floors function
     *
     * For a collection of people, return a vector of their destination
     * floors.  The floor indices should not necessarily be unique
     * as we may want to understand which is the most popular floor.
     */
    fn get_dest_floors(&self) -> Vec<usize> {
        //Initialize a new vector of usizes
        let mut dest_floors: Vec<usize> = Vec::new();

        //Loop through the vector of persons
        for pers in self.iter() {
            //Add the dest floor to the vector
            let dest_floor = pers.floor_to;
            dest_floors.push(dest_floor);
        }

        //Return the destination floors vector
        dest_floors
    }

    /** get_num_people function
     *
     * For a collection of people, return a usize describing how
     * many of them there are.
     */
     fn get_num_people(&self) -> usize {
        //Return the length of the vector
        self.len()
    }

    /** get_num_people_waiting function
     *
     * For a collection of people, return a usize describing how
     * many of them are currently waiting for the elevator.
     */
    fn get_num_people_waiting(&self) -> usize {
        //Initialize a usize counting the numper of people waiting
        let mut num_waiting: usize = 0_usize;

        //Loop through the vector of persons
        for pers in self.iter() {
            //Skip if the person is not waiting
            if pers.floor_on == pers.floor_to {
                continue;
            }

            //If the person is waiting, increment the counter
            num_waiting += 1_usize;
        }

        //Return the counter
        num_waiting
    }

    /** get_aggregate_wait_time function
     *
     * For a collection of people, return a usize counting the
     * total number of time steps they've been waiting.
     */
    fn get_aggregate_wait_time(&self) -> usize {
        //Initialize a usize for the number of time steps the people spent waiting
        let mut aggregate_wait_time: usize = 0_usize;

        //Loop through the vector of persons
        for pers in self.iter() {
            //Increment the usize with their wait time
            aggregate_wait_time += pers.wait_time;
        }

        //Return the usize
        aggregate_wait_time
    }

    /** are_people_going_to_floor function
     *
     * For a collection of people, return a boolean signifying whether
     * any of them are going to a given floor.
     */
    fn are_people_going_to_floor(&self, floor_index: usize) -> bool {
        //Initialize a boolean tracking if people are going to the given floor
        let mut is_going_to_floor: bool = false;

        //Loop through the people on the elevator and check
        for pers in self.iter() {
            //If the person is not going to the given floor then skip
            if pers.floor_to != floor_index {
                continue;
            }

            //Otherwise update the boolean and break
            is_going_to_floor = true;
            break;
        }

        //Return the is_going_to_floor boolean
        is_going_to_floor
    }

    /** are_people_waiting function
     *
     * For a collection of people, return a boolean signifying whether
     * any of them are waiting.
     */
    fn are_people_waiting(&self) -> bool {
        //Initialize a boolean tracking if people are waiting
        let mut is_waiting: bool = false;

        //Loop through the people and check if they are waiting
        for pers in self.iter() {
            //If the person is not waiting, then skip
            if pers.floor_on == pers.floor_to {
                continue;
            }

            //Otherwise update the boolean and break
            is_waiting = true;
            break;
        }

        //Return the is_going_to_floor boolean
        is_waiting
    }

    /** increment_wait_times function
     *
     * For a collection of people, increment their wait times.
     */
    fn increment_wait_times(&mut self) {
        //Loop through the people and increment their wait times
        for pers in self.iter_mut() {
            pers.increment_wait_time();
        }
    }

    /** reset_wait_times function
     *
     * For a collection of people, reset their wait times.
     */
    fn reset_wait_times(&mut self) {
        //Loop through the people and reset their wait times
        for pers in self.iter_mut() {
            pers.reset_wait_time();
        }
    }
}