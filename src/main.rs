extern crate rustc_serialize;
extern crate rand; 

use std::fs::File;
use std::io::prelude::*;
use rustc_serialize::Encodable;
use rustc_serialize::json::{self, Encoder};
use rand::Rng;
use rand::distributions::{IndependentSample, Range}; 

//create a file with new adresses every once in a while by
//Reverse Geocoding a city? This is a very hard thing to do when you
//have no access to real adresses 


//I sould put all these structs in a seperate file because this is insane 😅
#[derive(RustcDecodable, RustcEncodable, Debug)]
struct PackageDimentions {
    height: i64,
    width: i64,
    depth: i64,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
struct Package {
    id: i64,
    dimentions: PackageDimentions,
    fragile: bool, 
}

struct Name {
    first: String,
    insertion: String,
    last: String,
}

struct Address {
    street: String,
    street_number: i64,
    postial_code: String,
    city: String,
    country: String,
}

struct Consumer {
    id: i64,
    name: Name,
    birthday: i64, 
    gender: String,
    address: Address,
}

struct Driver {
    id: i64,
    name: Name,
    birthday: i64, 
    gender: String,
    years_of_exprience: i64, 
}

struct Delivery {
    id: i64,
    status: String,
    departure_time: i64,
    package: Package, 
    consumer: Consumer,
    driver: Driver,
}

struct Deliveries {
    deliveries: vec<Delivery>,
}

fn main() {
    generate_deliveries(); 
}

fn generate_package() -> Package {
    //find out dimention ranges & structure of package ID's from fedex mock data
    let range = Range::new(10, 150);
    let mut random = rand::thread_rng();

    let dimentions = PackageDimentions {
        height: range.ind_sample(&mut random),
        width: range.ind_sample(&mut random),
        depth: range.ind_sample(&mut random),
    };

    let package = Package {
        id: range.ind_sample(&mut random),
        dimentions: dimentions,
        fragile: random.gen::<bool>(),
    };

    return package; 
}

// fn generate_consumer() -> Consumer {

// }

// fn generate_driver() -> Driver {

// }

fn generate_deliveries() {
    //put stuff from structs in an object 
    let package = generate_package();
    let input = vec![package];

    encode_json(input);
}

//for now I'll use basic encoding instead of pretty encoding because I don't know how to write the file correctly
fn encode_json(input: Vec<Package>) {
    let json = json::encode(&input).unwrap();

    create_json_file(json);
}

fn create_json_file(json: String) {
    //TODO: look into fs::OpenOptions for appending lines into a file
    let mut file = File::create("MOCK_DATA.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();

    println!("done :)");
}
