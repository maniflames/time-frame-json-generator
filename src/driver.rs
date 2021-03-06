extern crate rand; 

use rand::Rng;
use utils;

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Driver {
    id: i64,
    name: utils::Name,
    birthday: i64, 
    gender: String,
    years_of_experience: i64, 
    departure_time: i64, 
}

pub fn generate() -> Driver {
    let mut random = rand::thread_rng();

    return Driver {
        id: random.gen_range(0, 100),
        name: utils::generate_name(),
        birthday: random.gen_range(0, 100), 
        gender: utils::generate_gender(),
        years_of_experience: random.gen_range(0, 70),
        departure_time: random.gen_range(1528070400, 1538070400),
    };
}