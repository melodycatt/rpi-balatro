//start here! lets follow the code as it takes us through all the files

//ignore this thing
//#![feature(generic_arg_infer)]

//first we import something from a library - code that someone else wrote,
//but it would get pretty repetitive if everyone wrote it themselves
//in fact, this is a special library: std, or standard. this comes with rust when you install it
//it includes all the simple operations that you *might* need,
//but they would just take up a lot of space if all the unneccesary ones 
//were always there in every project
//a lot of these operations, while simple, would take very specific knowledge to implement yourself

//here we have std::env::current_dir. what is this?
//we start with the 'crate' name: std (standard)
//a crate is the name for a library pretty much - it goes with the rust compiler "cargo"
//like when you type "cargo run"
//then we have a module within std, which is a subsection of the library
//modules are mostly semantic, but can also be optional in a crate, using 'features'
//further saving storage space.
//then we say what from the library we want to use in our library, here its current_dir
//current_dir is a function that takes 0 paramaters and returns a "PathBuf" containing the path of the current directory
//current_dir is generally the root folder of your crate - here its rpi-balatro/...
//you can see this info just by hovering over the name "current_dir"
//there you'll see a description and maybe even examples and extra info
//really, its usually just whatever folder the terminal is in (changed using `cd` like i showed you)
//when you run "cargo run"
//(by the way, cd literally stands for current directory. you might also see cwd which is current working directory)
//the only time its something else is when you dont run it through the terminal, but thats pretty rare-ish

//a PathBuf (PathBuffer) is struct (data structure) basically a fancy list of strings with some extra methods
//(methods are functions owned by types)
//that are specific to manipulating folder paths and stuff
//how do i know this? two ways:
//in the description of current_dir, where it mentions that it returns a PathBuf, its actually linked to the docs
//docs are.. documentation. they tell you what everything in a library is and how to use it
//so, i could just click that link and read it
//another way is find where PathBuf is, in this case its std::path::PathBuf, 
//and hover over that name to see the description
//now, we dont need to `use std::path::PathBuf;` because current_dir does that in its code
//we only need to do that if we want to create or use static methods from PathBuf
//static methods are methods called using Struct::method(), and they generally return an instance of that Struct
//for example String::from(&str) returns a new mutable String made out of an immutable &str
//the alternative are called from an instance of the struct, like this:
/*
let foo = Struct::new(); // new() generally creates a default, empty, or customisable (with paramaters) instance
foo.method(); // this is the non-static function, acting on data specific to the instance 'foo'
*/
//you can tell, because in their signatures (the names of the methods, their paramaters, and their return types)
//non-static methods will have a paramater that is simply `self`, `&self`, or `&mut self`
//`self` will take ownership of the Struct instance, either returning it afterwards
//or dropping it from memory at the end of the method 
//(this is useful if you want to create more borrows of it, because you cant borrow the same thing twice simultaneously)
//(if you did, you would have two things editing the same values at the same time, potentially causing unexpected behaviour)
//the other two, `&self` and `&mut self`, are just borrows of the value
//PathBuf is a buffer, which is a 'serial' list, as in you access it one value at a time
//(here, its a list of directories, going through them one at a time from the root of the computer)
use std::env::current_dir;
//technically we dont even need the current_dir part:
//normally we can refer to a crate without needing to `use` it
//so every time we wanted to use current_dir we would literally go `std::env::current_dir();`
//if we did `use std::env;`, we could then do `env::current_dir();`

//here we import one of our own modules
//not just that, we also import *, which means everything from the jokers module
//this means that instead of doing `use rpi_balatro::jokers;` and then, say `jokers::joker_loader();`,
//we can individually use everything in the module without the prefix `jokers::` and without listing it all
use rpi_balatro::jokers::*;
use rpi_balatro::game;
//modules we want in our crate (project) are defined in <crate_directory>/src/lib.rs
//lib stands for library, src stands for source (code). we're in <crate_directory>/src/main.rs right now;
//<crate_directory> in our case is rpi-balatro
//lets go to lib.rs for a second - its on the left in the same folder as this

//main.rs isnt a module. why?
//if you want to use cargo run, it will find main.rs, either in src or src/bin, and run the main() function
//every crate that isnt a library needs a main.rs
pub fn main() {
    //current_dir() doesnt *actually* return a PathBuf, it returns a *Result* containing one or an Error
    //you see, there are two ways errors are handled at runtime
    //(runtime is when you actually run the code, compiletime is when you compile it into binary to run it)
    //the first one is a panic. it stops the program and says some short information about what caused it
    //the second is an Error type. these can hold more specific information to the Error,
    //and there is usually an Error type specific to a certain struct, module, or crate
    //Errors are usually returned in Result types, which will either have an expected value
    //or an Error. to get the expected value, either call result.unwrap() or result.expect(error_message: &str)
    //however, calling this function on an Error result will panic, so be careful
    let cwd = current_dir();
    //as_path turns the PathBuf into a Path, the equivalent of a str when compared to a String
    //PathBuf is a owned (and therefor mutable) version of a Path, and therefor has more functionality
    //but takes more memory and speed
    println!("{:?}", cwd.unwrap().as_path());

    //here we use a function imported from rpi_balatro::jokers. command+click it to look into it
    let loader = joker_loader();
    let joker = loader.load_joker("Showman");
    println!("{:#?}, {}", joker, joker.apply(0.2, 0.3));
    joker.apply(0.2, 0.3);

    //ignore
}