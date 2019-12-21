use std::path::{Path, PathBuf};
use std::fmt;
use crate::hash::{Hash, hash_file};

#[derive(Debug)]
pub struct IndexedFile {
    path: PathBuf,
    hash: Hash
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
