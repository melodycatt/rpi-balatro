//you can ignore that bit youve already read below, if this is your second time here

//super, once again references the supermodule
//libloading is for those symbols we talked about earlier
//convert_case is for different casing styles, which i use to convert human joker names to ids for saving them;
//rust_uses_snake_casing
//manyLanguagesUseCamelCasing
//And-So-On
//std::fs stands for file service. for reading and writing files
use libloading::{Library, Symbol};
use super::{Joker, JokerData, JokerEnhancements};
use convert_case::{Case, Casing};
use serde_binary::{to_vec, from_vec, binary_stream::Endian};
use std::{env::{current_dir, current_exe}, fs};
use once_cell::sync::Lazy;

//this one is weird..
//Lazy is a special struct. to be honest, im not fully sure how it works.
//but basizally it has type, and is initialised with a function that returns that type
//when the Lazy is first referenced, it calls that function to create an instance of that type and stores it
//then, referencing the Lazy just works as if youre referncing the value the Lazy has
//this basically lets any type be a constant, where it couldnt usually be
//(mutability, sizing, etc)
//this is useful so that anything can reference that value and it has a static lifetime

//pub(super) means this is only public within this module's 'supermodule'
//load is a submodule of jokers, jokers is the supermodule of load
//the "x_x :: " in that expect error message is just a fun style thing
//x_x is a little dead face
pub(super) static JOKER_LOADER: Lazy<JokerLoader> = Lazy::new(|| JokerLoader::new().expect("x_x :: Failed to load library"));
//ok go back to jokers/mod.rs


pub type JokerApplyFunction = fn(f64, f64) -> f64;
//this is a struct for loading joker templates ive saved
//i do this so i dont need to hardcode the effects of each one,
//and instead i can compile them into a binary library
//that i can dynamically load the functions from into Symbols
pub struct JokerLoader {
    lib: Library,
}

impl JokerLoader {
    //this function is only called by the lazy above - its a bad idea to have two Libraries
    //referencing to the same compiled dynamic library at once and may even cause errorx
    fn new() -> Result<Self, String> {
        //first we get the dynamic library:
        //current_exe() for the path of the current compiled binary executable (usually different to current_dir())
        //(and usually crate/target/debug/binaryname or crate/target/release/binaryname (yes theres no file extension))
        //then we unwrap the result, get the parent 
        //(because its the path to the executable itself, and not the folder its in)
        //(the parent is just the second last part of the path, here the enclosing folder)
        //then we add libjokers.dylib to the end, 
        ///because the dynamic library should be put in the same folder as the binary
        let lib_path = current_exe().unwrap().parent().unwrap().join("libjokers.dylib");  // Use the appropriate path for your platform 

        //unsafe tells rust that it doesnt need to check for memory safety here
        //and well handle it. however this is unsafe
        //we only use this when necessary, not so we just dont have to worry about it
        //and MAKE SURE the code is memory safe
        //we only do this because the rust compiler cant guarantee that it is
        //in this case because were loading external code
        //but we might also want to do unsafe stuff that we guarantee ourselves wont cause problems
        //thats rare though
        unsafe {
            let lib = Library::new(lib_path).map_err(|e| format!("Failed to load library: {}", e))?;
            //Ok returns a Result with the success type
            //the ? mark returns an Error result with whatever precedes it, 
            //if that is an error (but it leaves Ok's untouched)
            Ok(Self { lib })
        }
    }

    pub fn load_joker(&self, joker_id: &str) -> Joker {
        //some more unsafe code, because we dont know if the the function were getting is safe
        //format! is a macro that works similar to println!, but instead of printing anything
        //it returns the String that println! would normally print
        let apply: Symbol<'_, JokerApplyFunction> = unsafe {
            self.lib.get(joker_id.as_bytes())
                .expect(format!("x_x :: failed to load {joker_id} apply function").as_str())
        };

        //we get the path to where the jokers are stored
        let data_path = current_dir().unwrap().join(format!("assets/jokers/{joker_id}.joker"));
        //and read the JokerData that should be stored there
        //its written in binary, but we cant really just have arbitrary binary
        //so we store it into a Vec of 8 bit numbers, which can be read as binary serially
        let data_bytes = fs::read(data_path).unwrap();
        //then we use serde_binary::from_vec() to reconstruct the JokerData from the binary Vec
        let data = from_vec(data_bytes, Endian::Little).unwrap();
        //return a new joker by declaring it with no semicolon
        Joker {
            data,
            apply
        }
    }

    //this function does the same as above without the jokerdata
    //only used for deserializing a Joker
    pub fn load_symbol(&self, joker_id: &str) -> Symbol<'_, JokerApplyFunction> {
        unsafe {
            self.lib.get(joker_id.as_bytes())
                .expect(format!("x_x :: failed to load {} apply function", joker_id).as_str())
        }
    }

    //for creating new jokers, only used for that
    //you can figure this out yourself
    pub fn create_joker(name: &str) {
        let j = JokerData {
            name: name.to_owned(),
            id: name.to_case(Case::Snake).to_owned(),
            enhancements: JokerEnhancements::default()
        };
        let bin_path = current_dir().unwrap().join(format!("assets/jokers/{}.joker", j.id));  // Use the appropriate path for your platform  
        let v = to_vec(&j, Endian::Little).expect("couldnt vec");
        println!("{:?}", v);
        fs::write(bin_path, v).expect("couldnt write");
    }
}

//lets go to cards.rs now