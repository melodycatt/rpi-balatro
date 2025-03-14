use std::env::current_dir;
use rpi_balatro::jokers::*;

fn main() {
    let cwd = current_dir();
    println!("{:?}", cwd.unwrap().as_path());
    rpi_balatro::jokers::save("Showman");
    let loader = JokerLoader::new().unwrap();
    let joker = loader.load_joker("Showman");
    println!("{:#?}, {}", joker, joker.apply(0.2, 0.3));
    joker.apply(0.2, 0.3);
}