use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Get the target directory path
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let crate_dir = Path::new(&manifest_dir);
    let profile = env::var("PROFILE").unwrap();
    let target_dir = crate_dir.join("target").join(profile);
    let source_path = crate_dir.join("jokers/target/release/libjokers.dylib");

    //println!("{:?}", env::var("CARGO_TARGET_DIR").unwrap());

    // Define the path to the dylib you want to copy

    // Define the destination path in the target directory
    let target_path = target_dir.join("libjokers.dylib");
    println!("{:?}, {:?}", target_path, source_path);
    // Copy the dylib to the target folder
    fs::copy(source_path, target_path).expect("cargo:warning=Failed to copy dylib: {}");

    // Ensure the dylib is linked properly
    println!("cargo:rerun-if-changed=./jokers/target/release/libjokers.dylib");
}
