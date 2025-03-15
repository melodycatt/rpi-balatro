//while you can put normal code in here, thats only ever good for one thing
//functions, structs, and traits (get to that later) that you want other files to be able to access
//(or other people, if your crate is a library)
//but which dont really deserve their own mdoule

//here we say 'pub' to mean other files can access the module,
//'mod' to say were declaring a module
//'cards', 'jokers' and 'game' to say which module.
//rust will infer where the module is:
//either you have a .rs file in the same directory as this one with that module name (e.g. src/cards.rs)
//or a folder in this same directory with that module and a `mod.rs` inside is (e.g. src/jokers/mod.rs)
//if both exist, it will give an error and ask you to change that
pub mod cards;
pub mod jokers;
pub mod game;
//multiple modules can also be written in one file:
/*
pub mod module_one {
    //code you would normally put in module_one.rs...
}
pub mod module_two {
    //code you would normally put in module_two.rs...
}
*/ 

//if you arent making a library, theres really no reason to use modules, other than for neatness
//its quite a lot to have all your code in one file..


//ok go back to main.rs