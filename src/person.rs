//Import libraries
use std::fmt;
use rand::Rng;
use rand::distributions::{Distribution, Standard, Uniform, Bernoulli};
use rand::seq::SliceRandom;

/** Person struct schema
 *
 * A person has a Bernoulli distribution which is sampled at each
 * time step to decide whether the person is leaving.  The person
 * also has a current and destination floor.  If the floors match
 * then the person is not waiting for an elevator.  If they do,
 * then the person is waiting for an elevator.
 */
pub struct Person {
    floor_on: usize,
    floor_to: usize,
    dst_out: Bernoulli,
    is_on_elevator: bool
}

/** Person constructor function
 *
 * Initialize a person given a probability of that person leaving
 * the building, the number of floors in the building, and an RNG
 * instance.
 *
 * The p_out is used to instantiate the distribution to sample
 * when determining whether that person is leaving.
 *
 * The num_floors and rng instance are used together to randomly
 * generate that person's destination floor on instatiation.
 */
pub fn from(p_out: f64, num_floors: usize, mut rng: &mut impl Rng) -> Person {
    let dst_to = Uniform::new(0_usize, num_floors);
    let mut floor_to: usize = dst_to.sample(&mut rng);
    Person {
        floor_on: 0_usize,
        floor_to: floor_to,
        dst_out: Bernoulli::new(p_out).unwrap(),
        is_on_elevator: false
    }
}

/** Person type implementation
 *
 * The following functions are implemented for the person type,
 * and are callable via
 *
 * //Example
 * let my_person: Person = person::from(0.5_f64, 5_usize, &mut rng);
 * let is_leaving: bool = my_person.is_leaving(&mut rng);
 */
impl Person {
    /** is_leaving function
     *
     * Decide whether the person is leaving the building.
     * If so, then update the person's floor_to value to 0.
     * Then return the is_leaving boolean.
     */
    pub fn is_leaving(&mut self, mut rng: &mut impl Rng) -> bool {
        let is_leaving: bool = self.dst_out.sample(&mut rng);
        if is_leaving {
            self.floor_to = 0_usize;
        }
        is_leaving
    }

    /** is_waiting function
     *
     * Decide whether the person is waiting for an elevator.
     * Return the is_leaving boolean.
     */
    pub fn is_waiting(&mut self) -> bool {
        let is_waiting: bool = if (self.floor_on != self.floor_to) && !self.is_on_elevator {
                true
            } else {
                false
            };
        is_waiting
    }

    /** is_on_elevator function
     *
     * Decide whether the person is on the elevator currently.
     * Return the is_on_elevator boolean.
     */
    pub fn is_on_elevator(&mut self) -> bool {
        self.is_on_elevator
    }

    /** get_floor_on function
     *
     * Get the floor on which the person currently is.
     * Return the floor_on usize.
     */
    pub fn get_floor_on(&mut self) -> usize {
        self.floor_on
    }

    /** get_floor_to function
     *
     * Get the floor to which the person would like to travel.
     * Return the floor_on usize.
     */
    pub fn get_floor_to(&mut self) -> usize {
        self.floor_to
    }

    /** set_on_elevator function
     *
     * Set whether the person is on the elevator or not.
     * Update the person's is_on_elevator bool with input bool.
     */
    pub fn set_on_elevator(&mut self, on_elevator: bool) {
        self.is_on_elevator = on_elevator;
    }

    /** set_floor_on function
     *
     * Set the person's floor.
     * Update the person's floor_on usize with input usize.
     */
     pub fn set_floor_on(&mut self, new_floor_on: usize) {
        self.floor_on = new_floor_on;
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_str: String = if (self.floor_on != self.floor_to) && !self.is_on_elevator {
            format!("Person  {} -> {} ", self.floor_on, self.floor_to)
        } else if self.is_on_elevator {
            format!("Person [{} -> {}]", self.floor_on, self.floor_to)
        } else {
            format!("Person  {}", self.floor_on)
        };
        f.write_str(&display_str)
    }
}