mod person;
mod people;
mod building;
mod elevator;
mod floor;

//Import source modules
use crate::person::Person;
use crate::people::People;
use crate::building::Building;
use crate::elevator::Elevator;
use crate::floor::Floor;

//Import libraries
use rand::Rng;
use rand::distributions::{Distribution, Standard, Uniform, Bernoulli};
use rand::seq::SliceRandom;
use std::{thread, time};

//Elevator logic function
fn update_elevator(building: &mut Building) -> i32 {
    //If the elevator is stopped then move people on and off the elevator
    if building.elevator.stopped {
        building.exchange_people_on_elevator();
    }

    //If stopped, check where to go next
    if building.elevator.stopped {
        //Find the nearest destination floor among people on the elevator
        let (nearest_dest_floor, min_dest_floor_dist): (usize, usize) = building.elevator.get_nearest_dest_floor();

        //If the nearest dest floor is identified, then update the elevator
        if min_dest_floor_dist != 0_usize {
            //Unstop the elevator and move toward the nearest dest floor
            if nearest_dest_floor > building.elevator.floor_on {
                return 1_i32;
            } else {
                return -1_i32;
            }
        }

        //Find the nearest waiting floor among people throughout the building
        let (nearest_wait_floor, min_wait_floor_dist): (usize, usize) = building.get_nearest_wait_floor();

        //If the nearest wait floor is identified, then update the elevator
        if min_wait_floor_dist != 0_usize {
            //Unstop the elevator and move toward the nearest dest floor
            if nearest_wait_floor > building.elevator.floor_on {
                return 1_i32;
            } else {
                return -1_i32;
            }
        }
    } else {
        //If moving down and on the bottom floor, then stop
        if !building.elevator.moving_up && building.elevator.floor_on == 0_usize {
            return 0_i32;
        }

        //If moving up and on the top floor, then stop
        if building.elevator.moving_up && building.elevator.floor_on == (building.floors.len() - 1_usize) {
            return 0_i32;
        }

        //If there are people waiting on the current floor, then stop
        if building.are_people_waiting_on_floor(building.elevator.floor_on) {
            return 0_i32;
        }

        //If there are people waiting on the elevator for the current floor, then stop
        if building.elevator.are_people_going_to_floor(building.elevator.floor_on) {
            return 0_i32;
        }
    }

    //If we make it this far without returning, then return the current state
    if building.elevator.stopped {
        return 0_i32;
    } else if building.elevator.moving_up {
        return 1_i32;
    } else {
        return -1_i32;
    }
}

//Main function
fn main() {
    //Initialize the building
    let mut building = Building::from(
        4_usize, //Number of floors
        0.2_f64, //Probability someone arrives
        5.0_f64, //Base energy spent moving elevator up
        2.5_f64, //Base energy spent moving elevator down
        0.5_f64  //Coefficient for energy spent by moving N people
    );

    //Initialize the RNG
    let mut rng = rand::thread_rng();
    
    //Loop until the numer of time steps are complete
    let time_steps: i32 = 200_i32;
    for i in 0..time_steps {
        //Generate people arriving and update the elevator
        building.gen_people_arriving(&mut rng);
        building.gen_people_leaving(&mut rng);
        let direction: i32 = update_elevator(&mut building);

        //Update the elevator based on the direction
        if direction > 0_i32 {
            println!("Elevator will move up");
            building.elevator.stopped = false;
            building.elevator.moving_up = true; //Move up
        } else if direction < 0_i32 {
            println!("Elevator will move down");
            building.elevator.stopped = false;
            building.elevator.moving_up = false; //Move down
        } else {
            println!("Elevator will stop");
            building.elevator.stopped = true; //Stop
        }

        //Move the elevator and the people on the elevator from the current floor
        let _new_floor_index = building.elevator.update_floor();

        //Print the rendered building status
        println!("{}", building);
        println!("{}", building.elevator.get_energy_spent());

        //Sleep for one second in between time steps
        let one_sec = time::Duration::from_millis(1000);
        thread::sleep(one_sec);
    }
}