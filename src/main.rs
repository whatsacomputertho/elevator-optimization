mod person;
mod people;
mod building;
mod elevator;
mod elevators;
mod floor;
mod floors;
mod cli;

//Import source modules
use crate::people::People;
use crate::building::Building;
use crate::elevators::Elevators;
use crate::floors::Floors;
use crate::cli::ElevatorCli;

//Import libraries
use std::{thread, time};
use std::io::{Write, stdout};
use crossterm::{terminal, cursor, QueueableCommand};
use clap::Parser;

//Main function
fn main() {
    //Parse the command line args
    let cli_args = ElevatorCli::parse();
    let num_floors: usize = match cli_args.floors {
        Some(x) => x as usize,
        None => 4_usize
    };
    let num_elevators: usize = match cli_args.elevators {
        Some(x) => x as usize,
        None => 2_usize
    };
    let expected_arrivals: f64 = match cli_args.arrivals {
        Some(x) => x as f64,
        None => 0.2_f64
    };

    //Initialize the building
    let mut building = Building::from(
        num_floors,
        num_elevators,
        expected_arrivals, //Probability someone arrives
        5.0_f64, //Base energy spent moving elevator up
        2.5_f64, //Base energy spent moving elevator down
        0.5_f64  //Coefficient for energy spent by moving N people
    );

    //Initialize the RNG and stdout
    let mut rng = rand::thread_rng();
    let mut stdout = stdout();
    
    //Loop until the numer of time steps are complete
    let time_steps: i32 = 1000_i32;
    for i in 0..time_steps {
        //Generate people arriving and leaving
        building.gen_people_arriving(&mut rng);
        building.gen_people_leaving(&mut rng);

        //Move people on and off the elevators and out of the building
        building.flush_first_floor();
        building.exchange_people_on_elevator();

        //Decide where to move next
        let elevator_decisions: Vec<i32> = update_elevator(&mut building);

        //Loop through the elevators and update their directions according to the decisions
        for (i, decision) in elevator_decisions.iter().enumerate() {
            if decision > &0_i32 {
                building.elevators[i].stopped = false;
                building.elevators[i].moving_up = true; //Move up
            } else if decision < &0_i32 {
                building.elevators[i].stopped = false;
                building.elevators[i].moving_up = false; //Move down
            } else {
                building.elevators[i].stopped = true; //Stop
            }

            //Move the elevator and the people on the elevator from the current floor
            let _new_floor_index = building.elevators[i].update_floor();
        }

        //Increment the wait times, update average energy, update dest probabilities
        let energy_spent: f64 = building.elevators.get_energy_spent();
        building.increment_wait_times();
        building.update_average_energy(i, energy_spent);
        building.update_dest_probabilities();

        //Print the rendered building status
        let building_str: String = String::from(building.to_string());
        let building_str_len = building_str.matches("\n").count() as u16;
        let _ = stdout.write_all(building_str.as_bytes());
        stdout.flush().unwrap();

        //Sleep for one second in between time steps
        let one_sec = time::Duration::from_millis(100_u64);
        thread::sleep(one_sec);

        //Reset the cursor and clear the previous console output
        if i < time_steps - 1 {
            stdout.queue(cursor::MoveUp(building_str_len)).unwrap();
            stdout.queue(cursor::MoveToColumn(0)).unwrap();
            stdout.queue(terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();
        }
    }
}

//Elevator logic function
fn update_elevator(building: &mut Building) -> Vec<i32> {
    //Initialize a vector of i32s representing the decisions made for each elevator
    let mut elevator_decisions: Vec<i32> = Vec::new();

    for elevator in building.elevators.iter() {
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
            let (nearest_wait_floor, min_wait_floor_dist): (usize, usize) = building.get_nearest_wait_floor(elevator.floor_on);

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
            if elevator.moving_up && elevator.floor_on == (building.floors.len() - 1_usize) {
                elevator_decisions.push(0_i32);
                continue;
            }

            //If there are people waiting on the current floor, then stop
            if building.are_people_waiting_on_floor(elevator.floor_on) {
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

    //Return the vector of decisions
    elevator_decisions
}