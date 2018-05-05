extern crate rustc_serialize;
extern crate rand; 

use std::fs::File;
use std::io::prelude::*;
use rustc_serialize::json::{self};
use rand::Rng;

mod utils;
mod package;
mod consumer; 
mod driver;

//create a file with new adresses every once in a while by
//Reverse Geocoding a city? This is a very hard thing to do when you
//have no access to real adresses 

#[derive(RustcDecodable, RustcEncodable, Debug)]
struct Delivery {
    id: i64,
    status: String,
    departure_time: i64,
    package: package::Package, 
    consumer: consumer::Consumer,
    driver: driver::Driver,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
struct Deliveries {
    deliveries: Vec<Delivery>,
}

fn main() {
    generate_deliveries(3); 
}

fn generate_deliveries(num_of_deliveries: i32) {
    let mut random = rand::thread_rng();
    let mut input : Vec<Delivery> = Vec::new();

    for _x in 0..num_of_deliveries {
        let delivery = Delivery {
            id: random.gen_range(0, 100),
            status: "At hub".to_string(),
            departure_time: random.gen_range(0, 100),
            package: package::generate(), 
            consumer: consumer::generate(),
            driver: driver::generate(),
        };

        input.push(delivery);
    }

    encode_json(input);
}

//for now I'll use basic encoding instead of pretty encoding because I don't know how to write the file correctly
//I could read the string until a new line character comes up, then append a line and move on to the next line
fn encode_json(input: Vec<Delivery>) {
    let json = json::encode(&input).unwrap();

    create_json_file(json);
}

fn create_json_file(json: String) {
    //TODO: look into fs::OpenOptions for appending lines into a file
    let mut file = File::create("MOCK_DATA.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();

    println!("done :)");
}
