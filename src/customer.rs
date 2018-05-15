extern crate rand; 

use rand::Rng;
use utils;

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Customer {
    id: i64,
    name: utils::Name,
    birthday: i64, 
    gender: String,
    address: utils::Address,
}

pub fn generate() -> Customer {
    let mut random = rand::thread_rng();

    return Customer {
        id: random.gen_range(0, 100),
        name: utils::generate_name(),
        birthday: random.gen_range(0, 100), //this should eventually have the range of a timestamp
        gender: utils::generate_gender(),
        address: utils::generate_address(),
    };
}