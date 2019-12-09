use std::path::Path;
mod indexed_file;

fn main() {
    let indexed_file = indexed_file::process_file(Path::new("./nope.txt"));
    println!("{:#?}", indexed_file);
    println!("Hello, world!");
}