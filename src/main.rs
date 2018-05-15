extern crate rustc_serialize;
extern crate rand; 

use std::fs::File;
use std::io::prelude::*;
use rustc_serialize::Encodable;
use rustc_serialize::json::{self, Encoder};
use rand::Rng;

mod utils;
mod package;
mod customer;
mod driver;

#[derive(RustcDecodable, RustcEncodable, Debug)]
struct DeliveryRound {
    id: i64,
    driver: driver::Driver,
    packages: Vec<package::Package>, 
    status: String,
    departure_time: i64, //this is the departure from the Hub 
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
struct Deliveries {
    deliveries: Vec<DeliveryRound>,
}

fn main() {
    generate_deliveries(100); 
}

fn generate_deliveries(num_of_packages: i32) {
    let mut random = rand::thread_rng();
    let mut input: Vec<DeliveryRound> = Vec::new();

    //TODO: spread the num of packages over the deliveries
    let delivery = DeliveryRound {
        id: random.gen_range(0, 100),
        driver: driver::generate(),
        packages: package::generate_vec(num_of_packages), 
        status: "At hub".to_string(),
        departure_time: 1526803200, //TODO: Make this variable 
    };

    input.push(delivery);

    let deliveries = Deliveries {
        deliveries: input
    };

    encode_json(deliveries);
}

//pretty printing: https://zsiciarz.github.io/24daysofrust/book/vol1/day6.html 
fn encode_json(input: Deliveries) {
   
   let mut json = String::new(); 
   {
        let mut encoder = Encoder::new_pretty(&mut json);
        input.encode(&mut encoder).expect("encoding error");
   }

    create_json_file(json);
}

fn create_json_file(json: String) {
    let mut file = File::create("MOCK_DATA.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();

    println!("done :)");
}
