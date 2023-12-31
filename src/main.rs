mod modules;

use std::env;

use crate::modules::file::read_file;
use crate::modules::shavian::{roman_to_ipa, ipa_to_shavian};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if let Some(file) = args.get(1) {
        match roman_to_ipa(file) {
            Ok(contents) => println!("{}", contents.get(0..4).unwrap()),
            Err(e) => println!("{e}")
            
        }

        match read_file(file) {
            Ok(contents) => println!("{contents}"),
            Err(e) => println!("{e}")
        };

        ipa_to_shavian("");
    } else {
        println!("### No file name was specified ###");
    }

}
