extern crate iron;

use iron::prelude::*;
use iron::status;

mod universe;

fn main() {
    let trimmed = 1;

    let mut universe = vec![];
    let mut neut = vec![0];
    let mut prot = vec![0];
    let mut elec = vec![0];

    universe::initialize_life(trimmed, &mut universe);

    Iron::new(|_: &mut Request| {
        let buffer = universe.len().to_string();
        String::from_utf8(buffer).unwrap();
        Ok(Response::with((status::Ok, "{}", )))
    }).http("localhost:3000").unwrap();
}
