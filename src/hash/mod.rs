use blake2::Blake2b;
use std::option::Option::{*};
use std::path::Path;

#[derive(Debug)]
pub struct Hash {
    hash: Blake2b,
}   

pub fn hash_file (_file: &Path) -> Option<Hash> {
    return Some(Hash{hash: Blake2b::default()});
}

