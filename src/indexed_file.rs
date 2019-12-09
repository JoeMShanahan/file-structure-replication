use std::path::{Path, PathBuf};
use std::option::Option::{None};
use sha2::Sha512;
use std::fmt;

#[derive(Debug)]
pub struct IndexedFile {
    path: PathBuf,
    hash: Sha512
}

impl fmt::Display for IndexedFile {
  fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
      write!(f, "IndexedFile{{ path: {:?}, hash: {:?} }}", self.path, self.hash)
  }
}

pub fn process_file(file: &Path) -> Option<IndexedFile> {
    let path = file.to_path_buf();
    let hash = hash_file(file)?;
    let indexed_file = IndexedFile { path, hash };
    return Some(indexed_file);
}

fn hash_file (_file: &Path) -> Option<Sha512> {
    return Some(Sha512::default());
}
