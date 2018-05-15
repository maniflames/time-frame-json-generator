extern crate rand; 

use rand::Rng;
use customer; 

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Package {
    id: i64,
    delivery_delay: i64,
    //arrival_time: i64, //TODO: Make this happen!!
    weight: i64, //max 100
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

    return Package {
        id: random.gen_range(0, 250),
        delivery_delay: random.gen_range(300, 1200), 
        weight: random.gen_range(0, 100),
        fragile: random.gen::<bool>(),
        dimensions: generate_dimentions(),
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

fn generate_dimentions() -> Dimensions {
    let mut random = rand::thread_rng();

    return Dimensions {
        height: random.gen_range(0, 250),
        width: random.gen_range(0, 50),
        depth: random.gen_range(0, 100),
    };
}