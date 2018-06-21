extern crate rand; 

use std::fs::File;
use std::io::Read; 
use rustc_serialize::json;
use rand::Rng;


#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Name {
    first: String,
    insertion: String,
    last: String,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct Address {
    street: String,
    street_number: i64,
    postal_code: String,
    city: String,
    country: String,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct AddressBuf {
    pub val: Address
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
struct AddressList {
    addresses: Vec<Address>,
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

//Don't know how to return, moving this
pub fn generate_address() -> Address {
    let mut f: File = File::open("address_list.json").unwrap(); 
    let mut json = String::new();
    f.read_to_string(&mut json);

    let address_list_json: AddressList = json::decode(&json).unwrap(); 
    let address_list: Vec<Address> = address_list_json.addresses; 

    let index = rand::thread_rng().gen_range(0, address_list.len()); 

    //you can't return an index value or a reference so creating a new 'instance' is the best way to go 
    //this is a memory thing I really have to learn
    return address_list[index].clone(); 
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