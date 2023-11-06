//Import libraries
use std::fmt;
use rand::Rng;
use rand::distributions::{Distribution, Standard, Uniform, Bernoulli};
use rand::seq::SliceRandom;

/** Person struct schema
 *
 * A person has a Bernoulli distribution which is sampled at each
 * time step to decide whether the person is leaving.  The person
 * also has a current and destination floor.
 */
pub struct Person {
    pub floor_on: usize,
    pub floor_to: usize,
    pub is_leaving: bool,
    pub wait_time: usize,
    dst_out: Bernoulli
}

/** Person type implementation
 *
 * The following functions are implemented for the person type,
 * and are callable via
 *
 * //Example
 * let my_person: Person = Person::from(0.5_f64, 5_usize, &mut rng);
 * let is_leaving: bool = my_person.is_leaving(&mut rng);
 */
impl Person {
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
            is_leaving: false,
            wait_time: 0_usize,
            dst_out: Bernoulli::new(p_out).unwrap()
        }
    }

    /** gen_is_leaving function
     *
     * Decide whether the person is leaving the building.
     * If so, then update the person's floor_to value to 0.
     * Then return the is_leaving boolean.
     */
    pub fn gen_is_leaving(&mut self, mut rng: &mut impl Rng) -> bool {
        //Check if the is_leaving boolean is true, if so return it
        if self.is_leaving {
            return self.is_leaving;
        }

        //If the person is not leaving, then randomly generate whether they wish to leave
        let pers_is_leaving: bool = self.dst_out.sample(&mut rng);
        if pers_is_leaving {
            self.floor_to = 0_usize;
            self.is_leaving = pers_is_leaving;
        }
        self.is_leaving
    }

    /** increment_wait_time function
     *
     * Increment the person's wait time counter
     */
    pub fn increment_wait_time(&mut self) {
        //Increment the person's wait time counter
        self.wait_time += 1_usize;
    }

    /** reset_wait_time function
     *
     * Reset the person's wait time counter, presumably
     * once they reach their destination floor.
     */
    pub fn reset_wait_time(&mut self) {
        //Reset the person's wait time counter
        self.wait_time = 0_usize;
    }
}

//Display trait implementation for a person
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_str: String = if self.floor_on != self.floor_to {
            format!("Person {} -> {}", self.floor_on, self.floor_to)
        } else {
            format!("Person {}", self.floor_on)
        };
        f.write_str(&display_str)
    }
}