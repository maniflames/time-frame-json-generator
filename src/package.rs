extern crate rand; 

use rand::Rng;

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Package {
    id: i64,
    dimentions: Dimentions,
    fragile: bool, 
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Dimentions {
    height: i64,
    width: i64,
    depth: i64,
}

pub fn generate() -> Package {
    //find out dimention ranges & structure of package ID's from fedex mock data
    let mut random = rand::thread_rng();

    return Package {
        id: random.gen_range(0, 250),
        dimentions: generate_dimentions(),
        fragile: random.gen::<bool>(),
    };
}

fn generate_dimentions() -> Dimentions {
    let mut random = rand::thread_rng();

    return Dimentions {
        height: random.gen_range(0, 250),
        width: random.gen_range(0, 50),
        depth: random.gen_range(0, 100),
    };
}