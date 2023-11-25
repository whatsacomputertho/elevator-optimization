//Import source modules
use crate::building::Building;
use crate::floors::Floors;
use crate::people::People;

//Implement standard/imported modules
use rand::rngs::ThreadRng;
use rand::distributions::{Distribution, Uniform};

/** ElevatorController trait
 *
 * A struct implementing the ElevatorController trait may
 * control the decisions of a building's elevators.
 */
pub trait ElevatorController {
    fn update_elevators(&mut self);
}

/** RandomController struct schema
 *
 * A RandomController has the following properties
 * - building (Building): A building being controlled by the controller
 * - floors_to (Vec<Option<usize>>): A list tracking the destination floors of each elevator
 * - dst_to (Uniform): A uniform distribution used for randomizing the destination floors
 * - rng (impl Rng): A random number generator for use in randomizing the elevator's dest floors
 *
 * It MUST implement the ElevatorController trait
 */
 pub struct RandomController {
    pub building: Building,
    floors_to: Vec<Option<usize>>,
    dst_to: Uniform<usize>,
    rng: ThreadRng
}

//Implement the RandomController interface
impl RandomController {
    /** RandomController constructor function
     *
     * Initialize a RandomController given a building and an RNG instance
     */
    pub fn from(building: Building, rng: ThreadRng) -> RandomController {
        //Get the number of floors and elevators in the building
        let num_floors: usize = building.floors.len();
        let num_elevators: usize = building.elevators.len();

        //Initialize the destination floors for the elevators
        let floors_to: Vec<Option<usize>> = {
            let mut tmp_floors_to: Vec<Option<usize>> = Vec::new();
            for _ in 0..num_elevators {
                tmp_floors_to.push(None);
            }
            tmp_floors_to
        };

        //Initialize the distribution for randomizing dest floors
        let dst_to: Uniform<usize> = Uniform::new(0_usize, num_floors);

        //Initialize the controller
        RandomController {
            building: building,
            floors_to: floors_to,
            dst_to: dst_to,
            rng: rng
        }
    }
}

//Implement the ElevatorController trait for the RandomController
impl ElevatorController for RandomController {
    /** update_elevators function
     *
     * Update the building's elevators so that they travel to randomly
     * generated floors
     */
    fn update_elevators(&mut self) {
        //Loop through the elevators in the building
        for (i, elevator) in self.building.elevators.iter_mut().enumerate() {
            //If the destination floor for the elevator is None, then randomize it
            let floor_to: usize = match self.floors_to[i] {
                Some(x) => x as usize,
                None => self.dst_to.sample(&mut self.rng)
            };

            //If the elevator is not on its destination floor, then move toward it
            if floor_to > elevator.floor_on {
                elevator.stopped = false;
                elevator.moving_up = true;
            } else if floor_to < elevator.floor_on {
                elevator.stopped = false;
                elevator.moving_up = false;
            //If the elevator is on its destination floor, then stop and set is destination floor to None
            } else {
                elevator.stopped = true;
                self.floors_to[i] = None;
            }

            //Update the elevator
            let _new_floor_index = elevator.update_floor();
        }
    }
}

/** NearestController struct schema
 *
 * A NearestController has the following properties
 * - building (Building): A building being controlled by the controller
 *
 * It MUST implement the ElevatorController trait
 */
pub struct NearestController {
    pub building: Building
}

//Implement the NearestController interface
impl NearestController {
    /** NearestController constructor function
     *
     * Initialize a NearestController given a building and an RNG instance
     */
    pub fn from(building: Building) -> NearestController {
        //Initialize the controller
        NearestController {
            building: building
        }
    }
}

//Implement the ElevatorController trait for the NearestController
impl ElevatorController for NearestController {
    /** update_elevators function
     *
     * Update the building's elevators so that they travel to the nearest
     * destination floors first, then nearest wait floors.  Also stop on
     * floors in the direction of the destination to service waiting people
     */
    fn update_elevators(&mut self) {
        //Initialize a vector of decisions for the elevators
        let mut elevator_decisions: Vec<i32> = Vec::new();

        //Loop through the elevators in the building
        for elevator in self.building.elevators.iter() {
            //If stopped, check where to go next
            if elevator.stopped {
                //Find the nearest destination floor among people on the elevator
                let (nearest_dest_floor, min_dest_floor_dist): (usize, usize) = elevator.get_nearest_dest_floor();

                //If the nearest dest floor is identified, then update the elevator
                if min_dest_floor_dist != 0_usize {
                    //Unstop the elevator and move toward the nearest dest floor
                    if nearest_dest_floor > elevator.floor_on {
                        elevator_decisions.push(1_i32);
                        continue;
                    } else {
                        elevator_decisions.push(-1_i32);
                        continue;
                    }
                }

                //Find the nearest waiting floor among people throughout the building
                let (nearest_wait_floor, min_wait_floor_dist): (usize, usize) = self.building.get_nearest_wait_floor(elevator.floor_on);

                //If the nearest wait floor is identified, then update the elevator
                if min_wait_floor_dist != 0_usize {
                    //Unstop the elevator and move toward the nearest dest floor
                    if nearest_wait_floor > elevator.floor_on {
                        elevator_decisions.push(1_i32);
                        continue;
                    } else {
                        elevator_decisions.push(-1_i32);
                        continue;
                    }
                }
            } else {
                //If moving down and on the bottom floor, then stop
                if !elevator.moving_up && elevator.floor_on == 0_usize {
                    elevator_decisions.push(0_i32);
                    continue;
                }

                //If moving up and on the top floor, then stop
                if elevator.moving_up && elevator.floor_on == (self.building.floors.len() - 1_usize) {
                    elevator_decisions.push(0_i32);
                    continue;
                }

                //If there are people waiting on the current floor, then stop
                if self.building.are_people_waiting_on_floor(elevator.floor_on) {
                    elevator_decisions.push(0_i32);
                    continue;
                }

                //If there are people waiting on the elevator for the current floor, then stop
                if elevator.are_people_going_to_floor(elevator.floor_on) {
                    elevator_decisions.push(0_i32);
                    continue;
                }
            }

            //If we make it this far without returning, then return the current state
            if elevator.stopped {
                elevator_decisions.push(0_i32);
                continue;
            } else if elevator.moving_up {
                elevator_decisions.push(1_i32);
                continue;
            } else {
                elevator_decisions.push(-1_i32);
                continue;
            }
        }

        //Loop through the elevator decisions and update the elevators
        for (i, decision) in elevator_decisions.iter().enumerate() {
            //Update the elevator direction
            if *decision > 0_i32 {
                self.building.elevators[i].stopped = false;
                self.building.elevators[i].moving_up = true;
            } else if *decision < 0_i32 {
                self.building.elevators[i].stopped = false;
                self.building.elevators[i].moving_up = false;
            } else {
                self.building.elevators[i].stopped = true;
            }

            //Update the elevator
            let _new_floor_index = self.building.elevators[i].update_floor();
        }
    }
}