use std::{process::Command, error::Error};

pub struct Shavian {
    pub name: String,
    pub char: char
}

pub fn eng_to_ipa(file: &str) -> Result<String, Box<dyn Error>> {
    let mut pwd = Command::new("pwd");
    let mut speak = Command::new("espeak-ng");

    let pwd = String::from_utf8(pwd.output()?.stdout)?;
    let pwd = pwd[..pwd.len()-1].to_string() + &file[1..];

    speak
        .arg("-f")
        .arg(pwd)
        .arg("--ipa")
        .arg("-q");


    Ok(String::from_utf8(speak.output()?.stdout)?)
}
