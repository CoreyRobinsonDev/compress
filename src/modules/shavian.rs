use std::{process::Command, error::Error, collections::HashMap, fs};

#[derive(Debug)]
pub struct PhonemeCharacter<'a> {
    ipa: &'a str,
    examples: [&'a str; 3]
}

#[derive(Debug)]
pub struct ShavianCharacter<'a> {
    name: &'a str,
    character: char,
    phoneme: PhonemeCharacter<'a>
}

pub const SHAVIAN_ALPHABET: [ShavianCharacter; 48] = [
    ShavianCharacter { name: "PEEP", character: '\u{10450}', phoneme: PhonemeCharacter { ipa: "\u{0070}", examples: ["p","",""] }}, 
    ShavianCharacter { name: "BIB", character: '\u{1045A}', phoneme: PhonemeCharacter { ipa: "\u{0062}", examples: ["b","",""] }},
    ShavianCharacter { name: "TOT", character: '\u{10451}', phoneme: PhonemeCharacter { ipa: "\u{0074}", examples: ["t","tt",""] }},
    ShavianCharacter { name: "DEAD", character: '\u{1045B}', phoneme: PhonemeCharacter { ipa: "\u{0064}", examples: ["d","dd",""] }},
    ShavianCharacter { name: "KICK", character: '\u{10452}', phoneme: PhonemeCharacter { ipa: "\u{006B}", examples: ["k","ck",""] }},
    ShavianCharacter { name: "GAG", character: '\u{1045C}', phoneme: PhonemeCharacter { ipa: "\u{0261}", examples: ["g","",""] }},
    ShavianCharacter { name: "FEE", character: '\u{10453}', phoneme: PhonemeCharacter { ipa: "\u{0066}", examples: ["f","",""] }},
    ShavianCharacter { name: "VOW", character: '\u{1045D}', phoneme: PhonemeCharacter { ipa: "\u{0076}", examples: ["v","",""] }},
    ShavianCharacter { name: "THIGH", character: '\u{10454}', phoneme: PhonemeCharacter { ipa: "\u{03B8}", examples: ["th","",""] }},
    ShavianCharacter { name: "THEY", character: '\u{1045E}', phoneme: PhonemeCharacter { ipa: "\u{00F0}", examples: ["th","",""] }},
    ShavianCharacter { name: "SO", character: '\u{10455}', phoneme: PhonemeCharacter { ipa: "\u{0073}", examples: ["s","ss",""] }},
    ShavianCharacter { name: "ZOO", character: '\u{1045F}', phoneme: PhonemeCharacter { ipa: "\u{007A}", examples: ["z","s",""] }},
    ShavianCharacter { name: "SURE", character: '\u{10456}', phoneme: PhonemeCharacter { ipa: "\u{0283}", examples: ["sh","ti",""] }},
    ShavianCharacter { name: "MEASURE", character: '\u{10460}', phoneme: PhonemeCharacter { ipa: "\u{0292}", examples: ["s","",""] }},
    ShavianCharacter { name: "CHURCH", character: '\u{10457}', phoneme: PhonemeCharacter { ipa: "\u{0074}\u{0283}", examples: ["ch","tch",""] }},
    ShavianCharacter { name: "JUDGE", character: '\u{10461}', phoneme: PhonemeCharacter { ipa: "\u{0064}\u{0292}", examples: ["g","dg",""] }},
    ShavianCharacter { name: "YEA", character: '\u{10458}', phoneme: PhonemeCharacter { ipa: "\u{006A}", examples: ["y","j",""] }},
    ShavianCharacter { name: "WOE", character: '\u{10462}', phoneme: PhonemeCharacter { ipa: "\u{0077}", examples: ["w","",""] }},
    ShavianCharacter { name: "HUNG", character: '\u{10459}', phoneme: PhonemeCharacter { ipa: "\u{014B}", examples: ["ng","n",""] }},
    ShavianCharacter { name: "HAHA", character: '\u{10463}', phoneme: PhonemeCharacter { ipa: "\u{0068}", examples: ["h","",""] }},
    ShavianCharacter { name: "LOLL", character: '\u{10464}', phoneme: PhonemeCharacter { ipa: "\u{006C}", examples: ["l","",""] }}, 
    ShavianCharacter { name: "ROAR", character: '\u{1046E}', phoneme: PhonemeCharacter { ipa: "\u{0072}", examples: ["r","",""] }},
    ShavianCharacter { name: "MIME", character: '\u{10465}', phoneme: PhonemeCharacter { ipa: "\u{006D}", examples: ["m","",""] }}, 
    ShavianCharacter { name: "NUN", character: '\u{1046F}', phoneme: PhonemeCharacter { ipa: "\u{006E}", examples: ["n","",""] }},
    ShavianCharacter { name: "IF", character: '\u{10466}', phoneme: PhonemeCharacter { ipa: "\u{026A}", examples: ["i","",""] }},
    ShavianCharacter { name: "EAT", character: '\u{10470}', phoneme: PhonemeCharacter { ipa: "\u{0069}\u{02D0}", examples: ["ee","e",""] }},
    ShavianCharacter { name: "EGG", character: '\u{10467}', phoneme: PhonemeCharacter { ipa: "\u{025B}", examples: ["e","",""] }},
    ShavianCharacter { name: "AGE", character: '\u{10471}', phoneme: PhonemeCharacter { ipa: "\u{0065}\u{026A}", examples: ["a","",""] }},
    ShavianCharacter { name: "ASH", character: '\u{10468}', phoneme: PhonemeCharacter { ipa: "\u{00E6}", examples: ["a","",""] }},
    ShavianCharacter { name: "ICE", character: '\u{10472}', phoneme: PhonemeCharacter { ipa: "\u{0061}\u{026A}", examples: ["i","ie",""] }},
    ShavianCharacter { name: "ADO", character: '\u{10469}', phoneme: PhonemeCharacter { ipa: "\u{0259}", examples: ["a","o",""] }},
    ShavianCharacter { name: "UP", character: '\u{10473}', phoneme: PhonemeCharacter { ipa: "\u{028C}", examples: ["u","",""] }},
    ShavianCharacter { name: "ON", character: '\u{1046A}', phoneme: PhonemeCharacter { ipa: "\u{0252}", examples: ["o","",""] }},
    ShavianCharacter { name: "OAK", character: '\u{10474}', phoneme: PhonemeCharacter { ipa: "\u{006F}\u{028A}", examples: ["oa","",""] }},
    ShavianCharacter { name: "WOOL", character: '\u{1046B}', phoneme: PhonemeCharacter { ipa: "\u{028A}", examples: ["oo","",""] }},
    ShavianCharacter { name: "OOZE", character: '\u{10475}', phoneme: PhonemeCharacter { ipa: "\u{0075}\u{02D0}", examples: ["oo","u",""] }},
    ShavianCharacter { name: "OUT", character: '\u{1046C}', phoneme: PhonemeCharacter { ipa: "\u{0061}\u{028A}", examples: ["ou","ow",""] }},
    ShavianCharacter { name: "OIL", character: '\u{10476}', phoneme: PhonemeCharacter { ipa: "\u{0254}\u{026A}", examples: ["oi","",""] }},
    ShavianCharacter { name: "AH", character: '\u{1046D}', phoneme: PhonemeCharacter { ipa: "\u{0251}\u{02D0}", examples: ["a","",""] }},
    ShavianCharacter { name: "AWE", character: '\u{10477}', phoneme: PhonemeCharacter { ipa: "\u{0254}\u{02D0}", examples: ["ough","au","augh"] }},
    ShavianCharacter { name: "ARE", character: '\u{10478}', phoneme: PhonemeCharacter { ipa: "\u{0251}\u{02D0}\u{0072}", examples: ["ar","",""] }},
    ShavianCharacter { name: "OR", character: '\u{10479}', phoneme: PhonemeCharacter { ipa: "\u{0254}\u{02D0}\u{0072}", examples: ["or","oar",""] }},
    ShavianCharacter { name: "AIR", character: '\u{1047A}', phoneme: PhonemeCharacter { ipa: "\u{025B}\u{0259}\u{0072}", examples: ["are","ar",""] }},
    ShavianCharacter { name: "ERR", character: '\u{1047B}', phoneme: PhonemeCharacter { ipa: "\u{025C}\u{02D0}", examples: ["ur","urr","or"] }},
    ShavianCharacter { name: "ARRAY", character: '\u{1047C}', phoneme: PhonemeCharacter { ipa: "\u{0259}\u{0072}", examples: ["er","ar","or"] }},
    ShavianCharacter { name: "EAR", character: '\u{1047D}', phoneme: PhonemeCharacter { ipa: "\u{026A}\u{0259}\u{0072}", examples: ["ear","er",""] }},
    ShavianCharacter { name: "IAN", character: '\u{1047E}', phoneme: PhonemeCharacter { ipa: "\u{0069}\u{0259}", examples: ["ia","",""] }},
    ShavianCharacter { name: "YEW", character: '\u{1047F}', phoneme: PhonemeCharacter { ipa: "\u{006A}\u{0075}\u{02D0}", examples: ["yew","",""] }}
];

pub fn text_stats(contents: &String) -> String {
    let num_char = contents
        .clone()
        .replace("\n", "")
        .chars()
        .count();

    let contents = contents
        .replace("\n", " \n ")
        .replace("  ", " ");

    let contents: Vec<&str> = contents
        .split(" ")
        .collect();

    let mut num_words = 0;

    for word in contents.iter() {
        if word != &"\n" && word != &"" { num_words += 1};
    }

    format!(
        "Contents: {}...\nCharacters: {}\nWords: {}",
        contents[0..4].join(" "),
        num_char,
        num_words
    )
}
pub fn ipa_cleanup(ipa: &String) -> String {
    let mut ipa = ipa
        .replace("\u{02CC}", "")
        .replace("\u{02C8}", "")
        .replace("\n", " ");

    ipa.push(' ');

    ipa
}

pub fn predictive_fix(ipa: char) -> char {
    let mut case: HashMap<char, ShavianCharacter> = HashMap::new();

    case.insert('\u{0279}', ShavianCharacter { name: "ROAR", character: '\u{1046E}', phoneme: PhonemeCharacter { ipa: "\u{0072}", examples: ["r","",""] }});
    case.insert('\u{0250}', ShavianCharacter { name: "ADO", character: '\u{10469}', phoneme: PhonemeCharacter { ipa: "\u{0259}", examples: ["a","o",""] }});
    case.insert('\u{0069}', ShavianCharacter { name: "EAT", character: '\u{10470}', phoneme: PhonemeCharacter { ipa: "\u{0069}\u{02D0}", examples: ["ee","e",""] }});
    case.insert('\u{0061}', ShavianCharacter { name: "AH", character: '\u{1046D}', phoneme: PhonemeCharacter { ipa: "\u{0251}\u{02D0}", examples: ["a","",""] }});
    case.insert('\u{025C}', ShavianCharacter { name: "UP", character: '\u{10473}', phoneme: PhonemeCharacter { ipa: "\u{028C}", examples: ["u","",""] }});
    
    match case.get(&ipa) {
        Some(c) => c.character,
        None => ipa
    }
}

// https://github.com/espeak-ng/espeak-ng/tree/master
pub fn roman_to_ipa(file: &str) -> Result<String, Box<dyn Error>> {
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

pub fn ipa_to_shavian(ipa: String) -> String {
    let mut shavian = String::new();
    let mut buffer: [u16; 3] = [0; 3];
    let mut skip2 = false;
    let mut skip1 = false;

    for (i, b) in ipa.encode_utf16().enumerate() {
        let mut flag = false;
        buffer[0] = buffer[1];
        buffer[1] = buffer[2];
        buffer[2] = b;

        if i == 0 || i == 1 { continue; };
        if skip2 { skip2 = false; skip1 = true; continue; };
        if skip1 { skip1 = false; continue; };

        if let Ok(join) = String::from_utf16(&buffer) {
            for shavian_obj in SHAVIAN_ALPHABET {
                if join == shavian_obj.phoneme.ipa {
                    flag = true;
                    skip2 = true;
                    shavian.push(shavian_obj.character);
                    break;
                }
            }
        }
        if !flag {
            if let Ok(join) = String::from_utf16(&buffer[..2]) {
                for shavian_obj in SHAVIAN_ALPHABET {
                    if join == shavian_obj.phoneme.ipa {
                        flag = true;
                        skip1 = true;
                        shavian.push(shavian_obj.character);
                        break;
                    }
                }
            }
        }
        if !flag {
            if let Ok(join) = String::from_utf16(&buffer[..1]) {
                for shavian_obj in SHAVIAN_ALPHABET {
                    if join == " ".to_string() { shavian.push(' '); break; }; 
                    if join == shavian_obj.phoneme.ipa {
                        flag = true;
                        shavian.push(shavian_obj.character);
                        break;
                    }
                }
                if !flag {
                    shavian.push(predictive_fix(join.chars().next().unwrap_or_else(|| '0')));
                }
            }
        }
    }

    shavian
}

pub fn roman_to_shavian(file: &str, flag: bool) -> Result<(), Box<dyn Error>> {
    let roman = fs::read_to_string(file)?;
    let ipa = roman_to_ipa(file)?;
    let shavian = ipa_to_shavian(ipa_cleanup(&ipa)); 

    if flag {
        let mut header = format!("###{}", file);
        let mut footer = String::new();

        loop {
            header += "#";
            if header.len() == 42 { break; }
        }

        for _ in 0..header.len() {
           footer += "#"; 
        }

        println!("{}", header);
        println!("===[ roman ]==============================");
        println!("{}", text_stats(&roman));
        println!("[-]=======================================\n");
        println!("===[ ipa ]================================");
        println!("{}", text_stats(&ipa));
        println!("[-]=======================================\n");
        println!("===[ shavian ]============================");
        println!("{}", text_stats(&shavian));
        println!("[-]=======================================");
        println!("{}", footer);
    } else {
        println!("{shavian}");
    }

    Ok(())
}
