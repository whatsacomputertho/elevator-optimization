mod person;
mod elevator;

//Import source modules
use crate::person::Person;
use crate::elevator::Elevator;

//Import libraries
use rand::Rng;
use rand::distributions::{Distribution, Standard, Uniform, Bernoulli};
use rand::seq::SliceRandom;
use std::{thread, time};

//Main function
fn main() {
    //Initialize the floors
    let num_floors: i32 = 2;
    let mut floors: Vec<Vec<Person>> = {
        let mut tmp_floors: Vec<Vec<Person>> = Vec::new();
        for i in 0..num_floors {
            let mut tmp_floor: Vec<Person> = Vec::new();
            tmp_floors.push(tmp_floor);
        }
        tmp_floors
    };

    //Initialize the elevator
    let mut my_elevator: Elevator = elevator::from(5.0_f64, 2.5_f64, 0.5_f64);
    println!("Elevator energy spent: {}", my_elevator.get_energy_spent(0_i32));

    //Initialize the probabilities & RNG
    let mut rng = rand::thread_rng();
    let p_in: f64 = 0.5_f64;
    let dst_in = Bernoulli::new(p_in).unwrap();
    
    //Loop until the numer of time steps are complete
    let time_steps: i32 = 200_i32;
    for i in 0..time_steps {
        //Resolve arrivals
        let is_arrival: bool = dst_in.sample(&mut rng);
        if is_arrival {
            let mut my_person: Person = person::from(0.5_f64, num_floors, &mut rng);
            println!("Person is arriving on floor {} and going to floor {}", my_person.get_floor_on(), my_person.get_floor_to());
        } else {
            println!("No arrivals");
        }

        //Sleep for one second in between time steps
        let one_sec = time::Duration::from_millis(1000);
        thread::sleep(one_sec);
    }
}