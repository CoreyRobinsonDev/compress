use std::{fs, error::Error};

pub fn read_file(file_name: &str) -> Result<String, Box<dyn Error>> 
{
    let contents = fs::read_to_string(file_name)?;
    let num_char = contents
        .clone()
        .replace("\n", "")
        .len();
    let contents = contents
        .replace("\n", " \n ")
        .replace("  ", " ");

    let contents: Vec<&str> = contents
        .split(" ")
        .collect();

    let mut num_words = 0;

    for word in contents.iter()
    {
        if word != &"\n" && word != &"" { num_words += 1};
    }

    let output = format!(
        "File: {}\nContents: {}...\nCharacters: {}\nWords: {}",
        file_name,
        contents[0..4].join(" "),
        num_char,
        num_words
    );


    Ok(output)
}
