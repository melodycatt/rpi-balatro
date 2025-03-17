mod load;
pub use load::JokerLoader;
use load::JOKER_LOADER;
//this is a tiny function. really, its just for cleanliness and good practice,
//so after i explain the one confusing part here, we'll command+click on load::JOKER_LOADER above
//to see whats really going on.
//so whats `'static`? well, when we store a borrow of a value inside a struct, 
//we want to know how long the value lives in memory to make sure we arent
//accessing and writing to that memory after its dropped and potentially causing corruption.
//when returning a borrow from a function, you can only ever return an &'static borrow
//&'static is a special lifetime, meaning the value lives as long as the program.
//this is because any other borrows will only be made on values defined in the function,
//and those will be dropped as soon as the function ends, making them useless as return values;
//the only other values a function are statics and consts, which can be accessed anywhere,
//and values passed into the function.
//therefore the only borrows a function can return are borrows passed into a parameters,
//and borrows of static values.
//we just return a borrow of JOKER_LOADER by referencing it with no semicolon
//ok, now cmd+click JOKER_LOADER above
pub fn joker_loader() -> &'static JokerLoader { &JOKER_LOADER }

//quite a few imports here! everything serde is for 'serializing' a type;
//that is, turning it into 'serial' data that can be stored in storage
//in this case im serializing it into binary. thats just what i chose, because its more fun that way
//everything else, ill explain when we get to it
use serde::ser::Serializer;
use serde::de::Deserializer;
use load::JokerApplyFunction;
use libloading::Symbol;
use serde::{Serialize, Deserialize};
use crate::cards::CardEdition;

//derive is a special marker.
//rust has things called traits. when a struct 'implements' a trait, its basically saying
//this struct is guaranteed to have the fields, methods, and types the trait defines
//this is so that you can create your own struct to pass into a function someone else made
//and they dont have to have somehow made a version of that function for your struct,
//you just have to implement a trait and they make the type of the parameter a 'generic' type
//(the ones denoted in the <> brackets)
//and make a constrait that the generic implements that trait
//Debug lets you disply the struct's data in text for debugging
//derive is a thing that automatically generates code that implements Debug for you when you compile,
//based on an algorithm
//(because implementing traits like this is a trek and incredibly common)
#[derive(Debug)]
//<'a> defines a lifetime for this struct. this is so that the Symbol below will also have
//AT LEAST the lifetime 'a, so it can guarantee it lives long enough
//you never actually say "this lives for 3 seconds"
//its just automatically determined at compiletime based on when its made and when it can be dropped
//so basically the lifetime is where in the code the value is dropped from memory
pub struct Joker<'a> {
    pub data: JokerData,
    //apply isnt public. heres why:
    //a symbol is a reference to a function from already compiled code.
    //it kinda acts like a normal function, except when its a inside a struct
    //you have to type `(struct.symbol)(...);` to run it
    //to make this easier, i just implement a method of the same name below
    //that is shorthand for those extra brackets.
    //this doesnt cause a name conflict because apply isnt visible outside of this module

    //since a symbol is a reference to a function bts, it needs a lifetime when in a struct
    //the second generic is a function type
    //function types define the signature of a function 'pointer' 
    //(a pointer to where the function is loaded in memory, implicitly defined by the name where the function is defined)
    //(i defined what a function signature is in main.rs)
    //cmd+click JokerApplyFunction to see what i mean, then come back
    apply: Symbol<'a, JokerApplyFunction>
}
impl<'a> Joker<'a> {
    pub fn apply(&self, x: f64, y: f64) -> f64 {
        (self.apply)(x, y)
    }
}

//Symbols cant be serialized, but i want to be able to serialize an entire game,
//(to save your progress)
//which includes Jokers, which include Symbols,
//so instead, i make a custom implementation of the Serialize trait (instead of deriving it)
//so that whenever something tries to serialize a Joker, the Joker skips itself
//and just returns the serialization of its data
impl<'a> Serialize for Joker<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.data.serialize(serializer)
    }
}

//Deserializing a game will expect the entire Joker to be serialized
//(if i was to derive Deserialize)
//so i need a custom Deserialize implementation that actually deserializes the data as a JokerData
//and creates the necessary symbol and constructs a new Joker
//ill get into joker_loader().load_symbol() later
impl<'a, 'de> Deserialize<'de> for Joker<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = JokerData::deserialize(deserializer)?;
        let apply = joker_loader().load_symbol(&data.id);

        Ok(Joker {
            data,
            apply,
        })
    }
}

//here, everything in JokerData implements Serialize by default so we can just derive it
//same goes for Deserialize and Debug
//(because these traits need to serialize, deserialize, and display the values inside them,
//those values also need to implement those traits)
#[derive(Serialize, Deserialize, Debug)]
pub struct JokerData {
    pub name: String,
    pub id: String,
    pub enhancements: JokerEnhancements,
}

//Default has a single, static method.. default(). 
//it returns an instance of the struct withe the "default" values
//primitives have predefined defaults, and for other things you can define you own
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct JokerEnhancements {
    pub modifiers: JokerModifiers,
    pub edition: CardEdition,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct JokerModifiers {
    pub eternal: bool,
    pub perishable: bool,
    pub rental: bool,
}

//lets go to src/jokers/load.rs,
//as thats a submodule of this