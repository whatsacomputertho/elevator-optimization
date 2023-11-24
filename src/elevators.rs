//Import source modules
use crate::elevator::Elevator;
use crate::person::Person;
use crate::people::People;

//Define elevators trait
pub trait Elevators {
    fn get_dest_floors(&self) -> Vec<usize>;

    fn get_energy_spent(&mut self) -> f64;

    fn flush_people_leaving_elevators(&mut self) -> Vec<Vec<Person>>;

    fn update_floors(&mut self);

    fn increment_wait_times(&mut self);
}

//Implement elevators trait for Vec<Elevators>
impl Elevators for Vec<Elevator> {
    /** get_dest_floors function
     *
     * Loop through the elevators and get their dest floors,
     * then consolidate the vectors into a single vector
     */
    fn get_dest_floors(&self) -> Vec<usize> {
        //Initialize a vector of usizes to track the overall dest floors
        let mut dest_floors: Vec<usize> = Vec::new();

        //Loop through the elevators and get the dest floor vectors
        for elevator in self.iter() {
            //Get the dest floors of the elevator
            let elevator_dest_floors: Vec<usize> = elevator.get_dest_floors();

            //Append the dest floors to the list of dest floors if not contained
            for dest_floor in elevator_dest_floors.iter() {
                if dest_floors.contains(dest_floor) {
                    continue;
                }
                dest_floors.push(*dest_floor);
            }
        }

        //Return the dest floors
        dest_floors
    }

    /** get_energy_spent function
     *
     * Calculate the energy spent across all elevators during
     * a time step.
     */
    fn get_energy_spent(&mut self) -> f64 {
        //Initialize an f64 to aggregate the total energy spent
        let mut energy_spent: f64 = 0.0_f64;

        //Loop through the elevators and calculate their energy spent
        for elevator in self.iter_mut() {
            let elevator_energy_spent: f64 = elevator.get_energy_spent();

            //Add the energy spent to the total
            energy_spent += elevator_energy_spent;
        }

        //Return the aggregate energy spent
        energy_spent
    }

    /** flush_people_leaving_elevators function
     *
     * Loop through each elevator and flush them of the people
     * leaving each elevator, appending them into a vector
     */
    fn flush_people_leaving_elevators(&mut self) -> Vec<Vec<Person>> {
        //Initialize a vector of vectors of people
        let mut people_leaving_elevators: Vec<Vec<Person>> = Vec::new();

        //Loop through all the elevators and flush the people leaving the elevator
        for elevator in self.iter_mut() {
            let people_leaving_elevator: Vec<Person> = elevator.flush_people_leaving_elevator();

            //Append to the list of people leaving the elevators
            people_leaving_elevators.push(people_leaving_elevator);
        }

        //Return the vector of vectors of people leaving each elevator
        people_leaving_elevators
    }

    /** update_floors function
     *
     * Loop through each elevator and update its floor
     */
    fn update_floors(&mut self) {
        for elevator in self.iter_mut() {
            elevator.update_floor();
        }
    }

    /** increment_wait_times function
     *
     * Loop through each elevator and increment the wait times of
     * the people on the elevator.
     */
    fn increment_wait_times(&mut self) {
        for elevator in self.iter_mut() {
            elevator.increment_wait_times();
        }
    }
}