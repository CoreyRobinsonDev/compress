use std::env;
use compress::read_file;

fn main()
{
    let args: Vec<String> = env::args().collect();

    if let Some(file) = args.get(1) 
    {
        match read_file(file)
        {
            Ok(contents) => println!("{contents}"),
            Err(e) => println!("{}", e.to_string())
        };
    } 
    else
    {
        println!("### No file name was specified ###");
    }
}
