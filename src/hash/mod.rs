use sha2::Sha512;
use std::option::Option::{*};
use std::path::Path;

#[derive(Debug)]
pub struct Hash {
    hash: Sha512,
}   

pub fn hash_file (_file: &Path) -> Option<Hash> {
    return Some(Hash{hash: Sha512::default()});
}

