extern crate rustc_serialize;
extern crate rand; 

use std::fs::File;
use std::io::prelude::*;
use rustc_serialize::json::{self};
use rand::Rng;
use rand::distributions::{IndependentSample, Range}; 

//create a file with new adresses every once in a while by
//Reverse Geocoding a city? This is a very hard thing to do when you
//have no access to real adresses 


//I should put all these structs in a seperate file because this is insane 😅
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

#[derive(RustcDecodable, RustcEncodable, Debug)]
struct Name {
    first: String,
    insertion: String,
    last: String,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
struct Address {
    street: String,
    street_number: i64,
    postial_code: String,
    city: String,
    country: String,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
struct Consumer {
    id: i64,
    name: Name,
    birthday: i64, 
    gender: String,
    address: Address,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
struct Driver {
    id: i64,
    name: Name,
    birthday: i64, 
    gender: String,
    years_of_exprience: i64, 
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
struct Delivery {
    id: i64,
    status: String,
    departure_time: i64,
    package: Package, 
    consumer: Consumer,
    driver: Driver,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
struct Deliveries {
    deliveries: Vec<Delivery>,
}

fn main() {
    generate_deliveries(3); 
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

fn pick_gender(random: bool) -> String {
    if random {
        return "Man".to_string();
    } else {
        return "Woman".to_string();
    }
}

fn generate_consumer() -> Consumer {
    let range = Range::new(10, 150);
    let mut random = rand::thread_rng();

    let person_name = Name {
        first: "Kerst".to_string(),
        insertion: "".to_string(),
        last: "Man".to_string(),
    };

    let person_address = Address {
        street: "SmileStreet".to_string(),
        street_number: 22,
        postial_code: "3030 AB".to_string(),
        city: "ChristCapital".to_string(),
        country: "North Pole".to_string(),
    };

    let consumer = Consumer {
        id: range.ind_sample(&mut random),
        name: person_name,
        birthday: range.ind_sample(&mut random), //this should eventually have the range of a timestamp
        gender: pick_gender(random.gen::<bool>()),
        address: person_address,
    };

    return consumer;
}

fn generate_driver() -> Driver {
    let range = Range::new(10, 150);
    let mut random = rand::thread_rng();

    let person_name = Name {
        first: "Rudolph".to_string(),
        insertion: "the".to_string(),
        last: "Rendeer".to_string(),
    };

    let driver = Driver {
        id: range.ind_sample(&mut random),
        name: person_name,
        birthday: range.ind_sample(&mut random), 
        gender: pick_gender(random.gen::<bool>()),
        years_of_exprience: range.ind_sample(&mut random), 
    };

    return driver;
}

fn generate_deliveries(num_of_deliveries: i32) {
    let range = Range::new(10, 150);
    let mut random = rand::thread_rng();
    let mut input : Vec<Delivery> = Vec::new();

    for _x in 0..num_of_deliveries {
        let gen_package = generate_package();
        let gen_consumer = generate_consumer(); 
        let gen_driver = generate_driver(); 

        let delivery = Delivery {
            id: range.ind_sample(&mut random),
            status: "At hub".to_string(),
            departure_time: range.ind_sample(&mut random),
            package: gen_package, 
            consumer: gen_consumer,
            driver: gen_driver
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
