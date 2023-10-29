mod person;
mod elevator;
mod floor;

//Import source modules
use crate::person::Person;
use crate::elevator::Elevator;
use crate::floor::Floor;

//Import libraries
use rand::Rng;
use rand::distributions::{Distribution, Standard, Uniform, Bernoulli};
use rand::seq::SliceRandom;
use std::{thread, time};

//Main function
fn main() {
    //Initialize the floors
    let num_floors: usize = 4_usize;
    let mut floors: Vec<Floor> = {
        let mut tmp_floors: Vec<Floor> = Vec::new();
        for i in 0_usize..num_floors {
            let mut tmp_floor: Floor = floor::new();
            tmp_floors.push(tmp_floor);
        }
        tmp_floors
    };

    //Initialize the elevator
    let mut my_elevator: Elevator = elevator::from(5.0_f64, 2.5_f64, 0.5_f64);

    //Initialize the probabilities & RNG
    let mut rng = rand::thread_rng();
    let p_in: f64 = 0.2_f64;
    let dst_in = Bernoulli::new(p_in).unwrap();
    
    //Loop until the numer of time steps are complete
    let time_steps: i32 = 200_i32;
    for i in 0..time_steps {
        //Resolve arrivals
        let mut arrivals: Vec<Person> = Vec::new();
        while dst_in.sample(&mut rng) {
            let mut new_person: Person = person::from(0.5_f64, num_floors, &mut rng);
            arrivals.push(new_person);
        }
        if arrivals.len() == 0 {
            println!("No new arrivals");
        } else {
            for new_person in arrivals.iter_mut() {
                let floor_to = new_person.get_floor_to();
                println!("New arrival for floor {}", floor_to);
            }
        }
        floors[0].extend(arrivals);

        //Elevator movement logic
        //
        //Set some initial variables to track the floor index and whether a movement decision has been made
        let floor_index: usize = my_elevator.get_floor_on();
        let mut elevator_direction_set: bool = false;
        let mut elevator_to_stop: bool = false;
        let mut elevator_to_move_up: bool = false;

        //If the elevator is stopped then move people on and off the elevator
        if my_elevator.is_stopped() {
            //Mutable borrow the current floor
            let mut current_floor: &mut Floor = &mut floors[floor_index];

            //Move people off the current floor
            let people_leaving_floor: Vec<Person> = current_floor.flush_people_entering_elevator();
            let people_leaving_elevator: Vec<Person> = my_elevator.flush_people_leaving_elevator();
            my_elevator.extend(people_leaving_floor);
            current_floor.extend(people_leaving_elevator);

            //If the current floor is the first floor, then flush the floor
            if floor_index == 0_usize {
                current_floor.flush_people_leaving_floor();
            }
        }

        //If stopped and not yet updated, check if people are still on the elevator
        if !elevator_direction_set && my_elevator.is_stopped() {
            //Borrow the current floor
            let mut current_floor: &mut Floor = &mut floors[floor_index];

            //Get the destination floors from the elevator
            let dest_floors: Vec<usize> = my_elevator.get_dest_floors();

            //Initialize variables to track the nearest destination floor with and the min
            //distance between here and a destination floor
            let mut nearest_dest_floor: usize = 0_usize;
            let mut min_dest_floor_dist: usize = 0_usize;

            //Calculate the distance between each dest floor and the current floor
            for dest_floor_index in dest_floors {
                let dest_floor_dist: usize = if floor_index > dest_floor_index {
                    floor_index - dest_floor_index
                } else {
                    dest_floor_index - floor_index
                };

                //Check whether this is less than the current minimum, or if no minimum
                //has been assigned yet (in which case it is 0_usize)
                if min_dest_floor_dist == 0_usize || dest_floor_dist < min_dest_floor_dist {
                    min_dest_floor_dist = dest_floor_dist;
                    nearest_dest_floor = dest_floor_index;
                }
            }

            //If the nearest dest floor is identified, then update the elevator
            if min_dest_floor_dist != 0_usize {
                println!("[{}] Nearest destination floor", nearest_dest_floor);

                //Unstop the elevator and move toward the nearest dest floor
                elevator_to_stop = false;
                if nearest_dest_floor > floor_index {
                    elevator_to_move_up = true;
                } else {
                    elevator_to_move_up = false;
                }
                elevator_direction_set = true;
            }
        }

        //If the elevator is stopped and still not updated then check for people waiting throughout the building
        if !elevator_direction_set && my_elevator.is_stopped() {
            //Initialize variables to track the nearest floor with waiting people and the min
            //distance between here and a floor with waiting people for comparison
            let mut nearest_wait_floor: usize = 0_usize;
            let mut min_wait_floor_dist: usize = 0_usize;

            //Loop through the floors and find the minimum distance floor with waiting people
            for (i, floor) in floors.iter_mut().enumerate() {
                //Initialize a variable to track if there are waiting people on this floor
                let mut is_wait_floor: bool = false;

                //Check if there is anyone waiting on the floor, if not then continue
                if !floor.are_people_waiting() {
                    continue;
                }

                //Calculate the distance between this floor and the waiting floor
                let wait_floor_dist: usize = if floor_index > i {
                    floor_index - i
                } else {
                    i - floor_index
                };

                //Check whether this is less than the current minimum, or if no minimum
                //has been assigned yet (in which case it is 0_usize)
                if min_wait_floor_dist == 0_usize || wait_floor_dist < min_wait_floor_dist {
                    min_wait_floor_dist = wait_floor_dist;
                    nearest_wait_floor = i;
                }
            }

            //If the nearest wait floor is identified, then update the elevator
            if min_wait_floor_dist != 0_usize {
                println!("[{}] Nearest floor with waiting people", nearest_wait_floor);

                //Unstop the elevator and move toward the nearest dest floor
                elevator_to_stop = false;
                if nearest_wait_floor > floor_index {
                    elevator_to_move_up = true;
                } else {
                    elevator_to_move_up = false;
                }
                elevator_direction_set = true;
            }
        }

        //If elevator is moving down and the current floor is the first floor, then stop the elevator
        if !(elevator_direction_set || my_elevator.is_stopped() || my_elevator.is_moving_up()) && floor_index == 0_usize {
            elevator_to_stop = true;
            elevator_direction_set = true;
        }

        //If elevator is moving up and the current floor is the top floor, then stop the elevator
        if !(elevator_direction_set || my_elevator.is_stopped()) && my_elevator.is_moving_up() && floor_index == 0_usize {
            elevator_to_stop = true;
            elevator_direction_set = true;
        }

        //If the elevator is moving then check for people waiting, if found then stop the elevator
        if !(elevator_direction_set || my_elevator.is_stopped()) {
            //Borrow the current floor
            let current_floor: &mut Floor = &mut floors[floor_index];

            //Check if there are people waiting on the current floor
            if current_floor.are_people_waiting() {
                println!("[{}] Stopping for people to get on", floor_index);
                elevator_to_stop = true;
                elevator_direction_set = true;
            } else if my_elevator.are_people_going_to_floor(floor_index) {
                println!("[{}] Stopping for people to get off", floor_index);
                elevator_to_stop = true;
                elevator_direction_set = true;
            }
        }

        //If the elevator should stop, then stop the elevator
        if elevator_to_stop || !elevator_direction_set {
            my_elevator.set_stopped(true);
        } else if elevator_to_move_up {
            my_elevator.set_stopped(false);
            my_elevator.set_moving_up(true);
        } else {
            my_elevator.set_stopped(false);
            my_elevator.set_moving_up(false);
        }

        //Calculate the number of people on the elevator
        let people_on_elevator: usize = my_elevator.get_num_people();

        //Move the elevator and the people on the elevator from the current floor
        let _new_floor_index = my_elevator.update_floor();

        //Calculate and print the energy spent
        let energy_spent: f64 = my_elevator.get_energy_spent();
        println!("Energy spent: {}", energy_spent);

        //Print the building status
        let mut building_status: String = String::new();
        for (i, floor) in floors.iter_mut().enumerate() {
            //Calculate the number of people on the floor but not on the elevator
            let mut people_on_floor = floor.get_num_people();

            //Initialize strings representing this floor
            let mut floor_roof: String = String::from("|---|");
            let mut floor_body: String = format!("| {} |", people_on_floor);

            //If this is the floor the elevator is on, then append the elevator as well
            if i == floor_index {
                //Initialize strings representing the elevator
                let elevator_roof: String = String::from("|---|");
                let elevator_body: String = format!("| {} |", people_on_elevator);

                //Append the elevator to the floor strings
                floor_roof.push_str(&elevator_roof);
                floor_body.push_str(&elevator_body);
            }

            //Add the floor to the building status
            building_status = [floor_roof, floor_body, building_status].join("\n");
        }
        //Print the rendered building status
        println!("{}", building_status);

        //Sleep for one second in between time steps
        let one_sec = time::Duration::from_millis(200);
        thread::sleep(one_sec);
    }
}