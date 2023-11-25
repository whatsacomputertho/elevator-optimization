mod person;
mod people;
mod building;
mod elevator;
mod elevators;
mod floor;
mod floors;
mod cli;
mod controller;

//Import source modules
use crate::building::Building;
use crate::elevators::Elevators;
use crate::floors::Floors;
use crate::cli::ElevatorCli;
use crate::controller::{ElevatorController, RandomController};

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
    let building = Building::from(
        num_floors,
        num_elevators,
        expected_arrivals,
        5.0_f64, //Base energy spent moving elevator up
        2.5_f64, //Base energy spent moving elevator down
        0.5_f64  //Coefficient for energy spent by moving N people
    );

    //Initialize the controller
    let controller_rng = rand::thread_rng();
    let mut controller = RandomController::from(
        building, controller_rng
    );

    //Initialize the RNG and stdout
    let mut rng = rand::thread_rng();
    let mut stdout = stdout();
    
    //Loop until the numer of time steps are complete
    let time_steps: i32 = 1000_i32;
    for i in 0..time_steps {
        //Generate people arriving and leaving
        controller.building.gen_people_arriving(&mut rng);
        controller.building.gen_people_leaving(&mut rng);

        //Move people on and off the elevators and out of the building
        controller.building.flush_first_floor();
        controller.building.exchange_people_on_elevator();

        //Update the elevators
        controller.update_elevators();

        //Increment the wait times, update average energy, update dest probabilities
        let energy_spent: f64 = controller.building.elevators.get_energy_spent();
        controller.building.increment_wait_times();
        controller.building.update_average_energy(i, energy_spent);
        controller.building.update_dest_probabilities();

        //Print the rendered building status
        let building_str: String = String::from(controller.building.to_string());
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