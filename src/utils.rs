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
    postal_code: String,
    city: String,
    country: String,
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

//create a file with new adresses every once in a while by
//Reverse Geocoding a city? This is a very hard thing to do when you
//have no access to real adresses 
//collecting adresses will be the hardest part
// this is probably the last thing we should try to automate

//for now I'll use a file with adresses from a random place in the netherlands
//gitignore both MOCK_DATA & the file containing adresses
// a random adress can be picked from the json file 
pub fn generate_address() -> Address {
    return Address {
        street: "SmileStreet".to_string(),
        street_number: 22,
        postal_code: "3030 AB".to_string(),
        city: "ChristCapital".to_string(),
        country: "North Pole".to_string(),
    };
}

//TODO: generate birthday
//TODO: generate_date_time(range)
//TODO: generate_departure_time

//TODO: create search function for find package by ID for carin?? 


pub fn generate_unique_ids(num_of_ids: usize) -> Vec<i64> {
    let mut ids: Vec<i64> = Vec::new(); 
    ids.push(90000); //If the vec is emtpy the 1st will never match

    while ids.len() < num_of_ids {
        let new_id = generate_unique_id(&ids);
        ids.push(new_id); 
   }     

   return ids;  
}

fn generate_unique_id(existing_ids: &Vec<i64>) -> i64 {
    let mut random = rand::thread_rng();
    let new_id: i64 = random.gen_range(100000000000, 999999999999); 

    if existing_ids.iter().position(|&x| x == new_id) == None {
        return new_id; 
    } else {
        return generate_unique_id(&existing_ids);
    }
}