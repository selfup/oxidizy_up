#![feature(test)]

extern crate iron;
extern crate rustc_serialize;
extern crate router;
extern crate staticfile;
extern crate mount;
extern crate test;

use test::Bencher;
use iron::prelude::*;
use iron::status;
use rustc_serialize::json;
use router::Router;
use std::path::Path;

mod universe;

fn main() {
    let mut router = Router::new();
    router.get("/api/v1/data", api_data);
    router.get("/", root);

    Iron::new(router).http("universe.rejs.io:80").unwrap();

    fn api_data(_req: &mut Request) -> IronResult<Response> {
        let trimmed = 5;

        let mut universe = vec![];
        let mut neut = vec![0];
        let mut prot = vec![0];
        let mut elec = vec![0];

        universe::initialize_life(trimmed, &mut universe);
        universe::particles(&mut universe, &mut neut, &mut prot, &mut elec);
        universe::charge_of_field(&mut prot, &mut elec, trimmed);
        universe::atom_charge(&mut universe);
        let encoded = json::encode(&universe).unwrap();

        Ok(Response::with((status::Ok, "{}", encoded)))
    }

    fn root(_req: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, Path::new("src/public/d3-visual.html"))))
    }
}

#[bench]
fn bench_universe_creation_calc(b: &mut Bencher) {
    let trimmed = 100;

    let mut universe = vec![];

    b.iter(|| universe::initialize_life(trimmed, &mut universe));
}
