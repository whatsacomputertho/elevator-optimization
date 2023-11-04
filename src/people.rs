//Import source modules
use crate::person::Person;

//Define people trait
pub trait People {
    fn get_dest_floors(&self) -> Vec<usize>;

    fn are_people_going_to_floor(&self, floor_index: usize) -> bool;
}

//Implement people trait for Vec<Person>
impl People for Vec<Person> {
    fn get_dest_floors(&self) -> Vec<usize> {
        //Initialize a new vector of usizes
        let mut dest_floors: Vec<usize> = Vec::new();

        //Loop through the vector of persons
        for pers in self.iter() {
            //Add the dest floor to the vector
            let dest_floor = pers.floor_to;
            dest_floors.push(dest_floor);
        }

        //Return the destination floors vector
        dest_floors
    }

    fn are_people_going_to_floor(&self, floor_index: usize) -> bool {
        //Initialize a boolean tracking if people are going to the given floor
        let mut is_going_to_floor: bool = false;

        //Loop through the people on the elevator and check
        for pers in self.iter() {
            //If the person is not going to the given floor then skip
            if pers.floor_to != floor_index {
                continue;
            }

            //Otherwise update the boolean and break
            is_going_to_floor = true;
            break;
        }

        //Return the is_going_to_floor boolean
        is_going_to_floor
    }
}