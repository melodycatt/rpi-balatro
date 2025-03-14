use libloading::{Library, Symbol};
use super::{Joker, JokerData, JokerEnhancements};
use convert_case::{Case, Casing};
use serde_binary::{to_vec, from_vec, binary_stream::Endian};

use std::{env::{current_dir, current_exe}, fs};

pub type JokerApplyFunction = fn(f64, f64) -> f64;
pub struct JokerLoader {
    lib: Library,
}

impl JokerLoader {
    // Load the shared library and initialize the JokerApplySymbolWrapper
    pub fn new() -> Result<Self, String> {
        let lib_path = current_exe().unwrap().parent().unwrap().join("libjokers.dylib");  // Use the appropriate path for your platform    
        unsafe {
            let lib = Library::new(lib_path).map_err(|e| format!("Failed to load library: {}", e))?;
            Ok(Self { lib })
        }
    }

    // Load the `my_function` from the shared library
    pub fn load_joker(&self, joker_id: &str) -> Joker {
        let apply: Symbol<'_, JokerApplyFunction> = unsafe {
            self.lib.get(joker_id.as_bytes())
                .expect(format!("x_x :: failed to load {} apply function", joker_id).as_str())
        };
        /*let apply = JokerApplyWrapper {
            function: *apply_function
        };*/
        let data_path = current_dir().unwrap().join(format!("assets/jokers/{joker_id}.joker"));  // Use the appropriate path for your platform  
        let data_bytes = fs::read(data_path).unwrap();
        let data = from_vec(data_bytes, Endian::Little).unwrap();
        Joker {
            data,
            apply
        }
    }
}

pub fn save(name: &str) {
    let j = JokerData {
        name: name.to_owned(),
        id: name.to_case(Case::Snake).to_owned(),
        enhancements: JokerEnhancements::default()
    };
    let bin_path = current_dir().unwrap().join(format!("assets/jokers/{}.joker", j.id));  // Use the appropriate path for your platform  
//    let x =serde_closure::Fn!(|a| a);
    let v = to_vec(&j, Endian::Little).expect("couldnt vec");
    println!("{:?}", v);
    std::fs::write(bin_path, v).expect("couldnt write");
}