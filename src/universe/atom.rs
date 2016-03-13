#[derive(RustcEncodable, Debug)]
pub struct Nucleus {
    pub protons:  i64,
    pub neutrons: i64,
}

#[derive(RustcEncodable, Debug)]
pub struct Atom {
    pub electrons: i64,
    pub nucleus: Nucleus,
}
