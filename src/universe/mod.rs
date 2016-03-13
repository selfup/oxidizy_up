extern crate rayon;
extern crate rand;

use self::rand::Rng;
use self::rayon::prelude::*;

mod atom;

#[derive(RustcEncodable, Debug)]
pub struct LifeBlock{
    pub x_y: (i64, i64),
    pub z: i64,
    pub charge: i64,
    pub atom: atom::Atom,
}

pub fn initialize_life(limit: i64, uni: &mut Vec<LifeBlock>) {
    for v in 0..limit + 1 {
        for w in 0..limit + 1 {
            for q in 0..limit + 1 {
                let n1: i64 = rand::thread_rng().gen_range(0, 118);
                let n2: i64 = rand::thread_rng().gen_range(0, 118);
                let n3: i64 = rand::thread_rng().gen_range(0, 118);
                uni.push(LifeBlock { x_y: (v, w), z: q,
                           charge: 0,
                           atom: atom::Atom { electrons: n1,
                                                nucleus: atom::Nucleus {protons: n2, neutrons: n3}
                                            }
                                   }
                        )
            }
        }
    }
}

#[test]
fn it_can_begin() {
    let mut universe = vec![];
    initialize_life(5, &mut universe);

    assert_eq!(universe.len(), 216);
    assert_eq!(universe[0].x_y, (0, 0));
    assert_eq!(universe[0].z, 0);
    assert_eq!(universe[20].x_y, (0, 3));
    assert_eq!(universe[20].z, 2);
}

pub fn particles(input: &mut Vec<LifeBlock>, n: &mut Vec<i64>, p: &mut Vec<i64>, e: &mut Vec<i64>) {
    n[0] = input.par_iter().map(|i| i.atom.nucleus.neutrons).sum();
    p[0] = input.par_iter().map(|i| i.atom.nucleus.protons).sum();
    e[0] = input.par_iter().map(|i| i.atom.electrons).sum();
}

#[test]
fn it_can_sense_the_field() {
    let mut universe = vec![];
    let mut neut = vec![0];
    let mut prot = vec![0];
    let mut elec = vec![0];
    initialize_life(1, &mut universe);
    particles(&mut universe, &mut neut, &mut prot, &mut elec);

    assert_eq!(universe.len(), 8);
    assert_eq!(neut.len(), 1);
    assert_eq!(prot.len(), 1);
    assert_eq!(elec.len(), 1);
}

pub fn charge_of_field(p: &mut  Vec<i64>, e: &mut Vec<i64>, u: i64) {
    let size = (u + 1) * (u + 1) * (u + 1);
    if p[0] == size && e[0] == size {
        println!("field has a neutral charge");
    } else if (p[0] > size) && (e[0] < p[0]) {
        println!("field is ionic");
    } else {
        println!("field is anionic");
    }
}

pub fn atom_charge(input: &mut Vec<LifeBlock>) {
    for i in input {
        if i.atom.nucleus.protons == i.atom.electrons {
            i.charge = 0;
        } else if i.atom.nucleus.protons > i.atom.electrons {
            i.charge = 1;
        } else {
            i.charge = -1;
        }
    };
}

#[test]
fn it_can_dictate_an_atoms_charge() {
    let mut universe = vec![];
    let mut neut = vec![0];
    let mut prot = vec![0];
    let mut elec = vec![0];
    let mut rand_nums = vec![0];
    let mut rando = "";
    initialize_life(5, &mut universe);
    particles(&mut universe, &mut neut, &mut prot, &mut elec);
    atom_charge(&mut universe);

    assert_eq!(universe.len(), 216);

    for u in universe {
        rand_nums.push(u.charge)
    }

    rand_nums.sort();
    rand_nums.dedup();

    if rand_nums.len() > 1 {
        rando = "random"
    }

    assert_eq!(rando, "random");
}
