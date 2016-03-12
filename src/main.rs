extern crate iron;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use rustc_serialize::json;

mod universe;

fn main() {
    let trimmed = 1;

    let mut universe = vec![];
    let mut neut = vec![0];
    let mut prot = vec![0];
    let mut elec = vec![0];

    universe::initialize_life(trimmed, &mut universe);
    universe::particles(&mut universe, &mut neut, &mut prot, &mut elec);
    universe::charge_of_field(&mut prot, &mut elec, trimmed);
    universe::atom_charge(&mut universe);

    println!("{:?}", universe);

    Iron::new(move |_: &mut Request| {

        // let serialized = serde_json::to_string(&universe).unwrap();

        let encoded = json::encode(&universe).unwrap();

        Ok(Response::with((status::Ok, "{}", encoded)))
    }).http("localhost:3000").unwrap();
}
