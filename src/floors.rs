//Import source modules
use crate::floor::Floor;
use crate::people::People;

//Import external/standard modules
use rand::Rng;

//Define floors trait
pub trait Floors {
    fn are_people_waiting_on_floor(&self, floor_index: usize) -> bool;

    fn get_nearest_wait_floor(&self, floor_on: usize) -> (usize, usize);

    fn get_dest_probabilities(&self) -> Vec<f64>;

    fn gen_people_leaving(&mut self, rng: &mut impl Rng);

    fn flush_first_floor(&mut self);

    fn increment_wait_times(&mut self);
}

//Implement people trait for Vec<Floor>
impl Floors for Vec<Floor> {
    /** are_people_waiting_on_floor function
     *
     * Check the Nth floor for people waiting.  Return a boolean
     * representing whether people are waiting on that floor.
     */
    fn are_people_waiting_on_floor(&self, floor_index: usize) -> bool {
        self[floor_index].are_people_waiting()
    }
    
    /** get_nearest_wait_floor function
     *
     * For a collection of floors, return a tuple containing the
     * nearest destination floor and the distance to it.
     */
    fn get_nearest_wait_floor(&self, floor_on: usize) -> (usize, usize) {
        //Initialize variables to track the nearest waiting floor and
        //the min distance between here and that floor
        let mut nearest_wait_floor: usize = 0_usize;
        let mut min_wait_floor_dist: usize = 0_usize;

        //Loop through the floors and find the minimum distance floor
        //with waiting people
        for (i, floor) in self.iter().enumerate() {
            //Check if there is anyone waiting on the floor, if not
            //then continue
            if !floor.are_people_waiting() {
                continue;
            }

            //Calculate the distance between this floor and the waiting
            //floor
            let wait_floor_dist: usize = if floor_on > i {
                floor_on - i
            } else {
                i - floor_on
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

    /** get_dest_probabilities function
     *
     * Loop through each floor and get each floor's dest_prob
     */
    fn get_dest_probabilities(&self) -> Vec<f64> {
        //Initialize a new vec of f64s
        let mut dest_probabilities: Vec<f64> = Vec::new();

        //Loop through the floors
        for floor in self.iter() {
            //Push the floor's dest_prob value into the vector
            dest_probabilities.push(floor.dest_prob);
        }

        //Return the vector
        dest_probabilities
    }

    /** gen_people_leaving function
     *
     * Given an RNG, generate people leaving based on their leaving
     * probability distribution.
     */
    fn gen_people_leaving(&mut self, mut rng: &mut impl Rng) {
        //Loop through the floors of the building
        for floor in self.iter_mut() {
            //Generate the people leaving on that floor
            floor.gen_people_leaving(&mut rng);
        }
    }

    /** flush_first_floor function
     *
     * Clear the first floor of anyone waiting to leave the building.
     */
    fn flush_first_floor(&mut self) {
        self[0].flush_people_leaving_floor();
    }

    /** increment_wait_times function
     *
     * Increment the wait times for all people throughout the building
     */
    fn increment_wait_times(&mut self) {
        for floor in self.iter_mut() {
            floor.increment_wait_times();
        }
    }
}