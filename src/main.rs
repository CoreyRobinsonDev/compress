mod modules;

use std::env;

use crate::modules::shavian::roman_to_shavian;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if let Some(file) = args.get(1) {
        let flag = match args.get(2) {
           Some(s) => *s == "--summary",
           None => false
        };

        if let Err(e) = roman_to_shavian(file, flag) {
            eprintln!("### {e} ###");
        };
    } else {
        eprintln!("### No file name was specified ###");
    }

}
