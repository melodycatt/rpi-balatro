use std::{path::Path, process::Command};
use rpi_balatro::jokers::save;

const NAME: &str = "Showman";

fn main() {
    let status = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .current_dir(Path::canonicalize(Path::new("./jokers")).unwrap())
        .status()
        .expect("msg");
    save(NAME);
    println!("{:?}", status.success());
}