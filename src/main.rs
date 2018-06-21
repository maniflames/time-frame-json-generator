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
    generate_deliveries(250); 
}

  //TODO: figure out how to generate the right amount of deliveries based on the number of packages 
fn generate_deliveries(num_of_packages: i32) {
    let mut random = rand::thread_rng();
    let mut input: Vec<DeliveryRound> = Vec::new();
    let mut package_ids: Vec<i64> = utils::generate_unique_ids(num_of_packages as usize); 

    let num_of_deliveries = random.gen_range(4, 8);
    let num_rest_packages = num_of_packages % num_of_deliveries;
    let num_packages_per_delivery = num_of_packages / num_of_deliveries;
    
    for _x in 0..num_of_deliveries {
        let input_packages;
        if _x + 1 == num_of_deliveries {
            input_packages = package::generate_vec(num_packages_per_delivery + num_rest_packages, &package_ids);
        } else {
            let mut package_ids_len = package_ids.len();
            let saved_package_ids: Vec<i64> = package_ids.split_off(&package_ids_len - (num_packages_per_delivery as usize));
            input_packages = package::generate_vec(num_packages_per_delivery, &saved_package_ids);
        }

        let delivery = DeliveryRound {
            id: random.gen_range(0, 100),
            driver: driver::generate(),
            packages: input_packages, 
            status: "At hub".to_string(),
            departure_time: 1529977246, //TODO: Make this variable 
        };

        input.push(delivery);
    }

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
