//Import libraries
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
    floor_on: i32,
    floor_to: i32,
    dst_out: Bernoulli
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
pub fn from(p_out: f64, num_floors: i32, mut rng: &mut impl Rng) -> Person {
    let dst_to = Uniform::new(0, num_floors);
    let floor_to: i32 = dst_to.sample(&mut rng);
    Person {
        floor_on: 0_i32,
        floor_to: floor_to,
        dst_out: Bernoulli::new(p_out).unwrap()
    }
}

/** Person type implementation
 *
 * The following functions are implemented for the person type,
 * and are callable via
 *
 * //Example
 * let my_person: Person = person::from(0.5_f64, 5_i32, rng);
 * let is_leaving: bool = my_person.is_leaving(rng);
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
            self.floor_to = 0;
        }
        is_leaving
    }

    /** is_waiting function
     *
     * Decide whether the person is waiting for an elevator.
     * Return the is_leaving boolean.
     */
    pub fn is_waiting(&mut self) -> bool {
        let is_waiting: bool = if self.floor_on != self.floor_to {
                true
            } else {
                false
            };
        is_waiting
    }

    /** get_floor_on function
     *
     * Get the floor on which the person currently is.
     * Return the floor_on i32.
     */
    pub fn get_floor_on(&mut self) -> i32 {
        self.floor_on
    }

    /** get_floor_to function
     *
     * Get the floor to which the person would like to travel.
     * Return the floor_on i32.
     */
    pub fn get_floor_to(&mut self) -> i32 {
        self.floor_to
    }
}