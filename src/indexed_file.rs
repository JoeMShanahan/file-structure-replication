use std::path::{Path, PathBuf};
use std::option::Option::{None};
use sha2::Sha512;

struct IndexedFile {
    path: PathBuf,
    hash: Sha512
}

fn processFile(file: &Path) -> Option<IndexedFile> {
  return None;
}