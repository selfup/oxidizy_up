#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

#[derive(Serialize, Debug)]
pub struct Nucleus {
    pub protons:  i64,
    pub neutrons: i64,
}

#[derive(Serialize, Debug)]
pub struct Atom {
    pub electrons: i64,
    pub nucleus: Nucleus,
}
