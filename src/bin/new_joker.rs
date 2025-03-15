use std::{path::Path, process::Command};
use rpi_balatro::jokers::create;

const NAME: &str = "showman";

fn main() {
    let status = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .current_dir(Path::canonicalize(Path::new("./jokers")).unwrap())
        .status()
        .expect("msg");
    create(NAME);
    println!("{:?}", status.success());
}