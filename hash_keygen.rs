use sha2::{Sha256, Digest};
use std::env;

fn main() {

    // Get cli args
    let args: Vec<String> = env::args().collect();

    // Check if there's exactly 1 arg
    if args.len() != 2 {
        println!("Usage: hash_keygen <key>");
        std::process::exit(1);
    }

    // Hash the key
    let mut key_hasher = Sha256::new();
    key_hasher.update(b"hello world");
    let key_hash = key_hasher.finalize();

    // Convert hash to hex digest
    let mut key_hexdigest = String::new();
    for i in key_hash {
        key_hexdigest.push_str(&format!("{:x}", i).to_string());
    }

    // Hash the arg
    let mut arg_hasher = Sha256::new();
    arg_hasher.update(&args[1]);
    let arg_hash = arg_hasher.finalize();

    // Convert hash to hex digest
    let mut arg_hexdigest = String::new();
    for i in arg_hash {
        arg_hexdigest.push_str(&format!("{:x}", i).to_string());
    }

    println!("Key hexdigest: {}", key_hexdigest);
    println!("Input hexdigest: {}", arg_hexdigest);

    // Compare hashes of key and arg
    if key_hexdigest == arg_hexdigest {
        println!("Unlocked!");
    }
    else {
        println!("Wrong key!");
    }
}
