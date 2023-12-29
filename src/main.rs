mod modules;

use std::env;

use crate::modules::file::read_file;
use crate::modules::shavian::eng_to_ipa;

fn main() {
    let args: Vec<String> = env::args().collect();
    

    if let Some(file) = args.get(1) {
        match eng_to_ipa(file) {
            Ok(contents) => println!("{contents}"),
            Err(e) => println!("{e}")
            
        }
        match read_file(file) {
            Ok(contents) => println!("{contents}"),
            Err(e) => println!("{e}")
        };
    } else {
        println!("### No file name was specified ###");
    }

}
