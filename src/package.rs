extern crate rand; 

use rand::Rng;
use customer; 

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Package {
    id: i64,
    delivery_delay: i64, //minutes
    //arrival_time: i64, //TODO: Make this happen!!
    weight: i64,
    fragile: bool, 
    dimensions: Dimensions,
    customer: customer::Customer,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Dimensions {
    height: i64,
    width: i64,
    depth: i64,
}

pub fn generate() -> Package {
    //find out dimention ranges & structure of package ID's from fedex mock data
    let mut random = rand::thread_rng();

    let package_weight : i64 = random.gen_range(0, 68); //kilos
    let is_fragile : bool = random.gen::<bool>();

    return Package {
        id: random.gen_range(0, 250),
        delivery_delay: generate_delivery_delay(package_weight, is_fragile),
        weight: package_weight,
        fragile: is_fragile,
        dimensions: generate_dimensions(),
        customer: customer::generate(), //TODO: find a way to link multiple packages to the same customer
    };
}

pub fn generate_vec(num_of_packages: i32) -> Vec<Package> {
    let mut packages : Vec<Package> = Vec::new();

    for _x in 0..num_of_packages {
        let package = generate();
        packages.push(package);
    }

    return packages;
}

fn generate_dimensions() -> Dimensions {
    let mut random = rand::thread_rng();

    return Dimensions {
        height: random.gen_range(0, 250),
        width: random.gen_range(0, 50),
        depth: random.gen_range(0, 100),
    };
}

fn generate_delivery_delay(weight : i64, fragile : bool) -> i64 {
    let multiplier : f64 = 0.1233;
    let casted_weight : f64 = weight as f64;
    let weight_delay : f64 = multiplier * casted_weight.powf(2.0); 
    let weight_delay_sec : i64 = weight_delay.round() as i64;

    let mut fragile_delay;

    //NOTE: this could obviously be a match 
    if fragile {
        fragile_delay = 1;
    } else {
        fragile_delay = 0;
    }

    return weight_delay_sec + (30 * fragile_delay);
}