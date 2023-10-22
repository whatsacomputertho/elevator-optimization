/** Elevator struct schema
 *
 * An elevator has the following properties
 * - energy_up (f64): Base energy spent per floor when empty & moving up
 * - energy_down (f64): Base energy spent per floor when empty & moving down
 * - energy_coef (f64): Multiplier for calculating energy spent while traveling with people
 * - floor_on (i32): The floor that the elevator is currently on
 * - moving_up (bool): If true, the elevator is moving up, else it is moving down
 * - stopped (bool): If true, the elevator is stopped, else it is moving
 */
pub struct Elevator {
    energy_up: f64,
    energy_down: f64,
    energy_coef: f64,
    floor_on: i32,
    moving_up: bool,
    stopped: bool
}

/** Elevator constructor function
 *
 * Initialize an elevator given its energy values, those being
 * energy spent traveling up and down, as well as the energy
 * coefficient/multiplier for when people are on the elevator
 *
 * The floor_on, moving_up, and stopped attributes are initialized
 * to 0_i32, true, and true respectively.
 */
 pub fn from(energy_up: f64, energy_down: f64, energy_coef: f64) -> Elevator {
    Elevator {
        energy_up: energy_up,
        energy_down: energy_down,
        energy_coef: energy_coef,
        floor_on: 0_i32,
        moving_up: false,
        stopped: true
    }
}

/** Elevator type implementation
 *
 * The following functions are implemented for the Elevator type,
 * and are callable via
 *
 * //Example
 * let my_elevator: Elevator = elevator::from(5.0_f64, 2.5_f64, 0.5_f64);
 * let is_leaving: bool = my_person.is_leaving(&mut rng);
 */
impl Elevator {
    /** get_energy_spent function
     *
     * Calculate the energy spent while the elevator is moving.
     * Accept the number of people currently on the elevator.
     * Return the total energy spent moving one floor.
     */
    pub fn get_energy_spent(&mut self, num_people: i32) -> f64 {
        let energy_spent = if self.stopped {
                0.0_f64
            } else if self.moving_up {
                self.energy_up + self.energy_coef * (num_people as f64)
            } else {
                self.energy_down + self.energy_coef * (num_people as f64)
            };
        energy_spent
    }
}