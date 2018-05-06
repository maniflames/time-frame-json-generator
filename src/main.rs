extern crate rustc_serialize;
extern crate rand; 

use std::fs::File;
use std::io::prelude::*;
use rustc_serialize::Encodable;
use rustc_serialize::json::{self, Encoder};
use rand::Rng;

mod utils;
mod package;
mod consumer; 
mod driver;

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
    let mut input: Vec<Delivery> = Vec::new();

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
    };

    let deliveries = Deliveries {
        deliveries: input
    };

    encode_json(deliveries);
}

//pretty printing: https://zsiciarz.github.io/24daysofrust/book/vol1/day6.html 
fn encode_json(input: Deliveries) {
   
   let mut json = String::new(); 
   {
        let mut encoder = Encoder::new_pretty(&mut string);
        input.encode(&mut encoder).expect("encoding error");
   }

    create_json_file(json);
}

fn create_json_file(json: String) {
    let mut file = File::create("MOCK_DATA.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();

    println!("done :)");
}
