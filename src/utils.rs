extern crate rand; 

use rand::Rng;

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Name {
    first: String,
    insertion: String,
    last: String,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Address {
    street: String,
    street_number: i64,
    postial_code: String,
    city: String,
    country: String,
}

pub fn test() {
    println!("{:?}", "What's good world?!?!");
}

pub fn generate_gender() -> String {
    if rand::thread_rng().gen::<bool>() {
        return "Man".to_string();
    } else {
        return "Woman".to_string();
    }
}

pub fn generate_name() -> Name {
    return Name {
        first: "Kerst".to_string(),
        insertion: "".to_string(),
        last: "Man".to_string(),
    };
}

pub fn generate_address() -> Address {
    return Address {
        street: "SmileStreet".to_string(),
        street_number: 22,
        postial_code: "3030 AB".to_string(),
        city: "ChristCapital".to_string(),
        country: "North Pole".to_string(),
    };
}