mod modules;

use std::env;

use crate::modules::file::read_file;
use crate::modules::shavian::roman_to_shavian;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if let Some(file) = args.get(1) {
        if let Ok(content) = read_file(file) {
            println!("############################\n");
            println!("{content}");
            if let Ok(shavian) = roman_to_shavian(file) {
                println!("{shavian}")
            } else {
                println!("### Error converting Romantic to Shavian alphabet ###");
            }
            println!("############################");
        } else {
            println!("### Error reading file ###");
        };
    } else {
        println!("### No file name was specified ###");
    }

}
