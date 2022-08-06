use std::{fs};

fn main() {
    
    let dirs: fs::ReadDir = fs::read_dir(".").unwrap();

    for dir in dirs {
        println!("{}", dir.unwrap().path().display());
    }
}
