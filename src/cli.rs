//Import library modules
use clap::{Parser};

/** ElevatorCli struct schema
 *
 * The ElevatorCli struct is used to store the command line
 * arguments passed into the application/
 */
#[derive(Parser)]
#[command(name="Elevator Optimization")]
#[command(author="whatsacomputertho")]
#[command(version="0.1.0")]
#[command(
    about="Simulate an elevator, measure wait time and energy usage",
    long_about="The Elevator Optimization CLI implements elevator simulation logic. \
                It models people arriving and leaving to and from the building, and \
                measures the elevator's energy usage, as well as average wait time \
                throughout the building.  The objective is to minimize with respect \
                to these measurements under various conditions."
)]
pub struct ElevatorCli {
    #[arg(long="floors")]
    pub floors: Option<usize>
}