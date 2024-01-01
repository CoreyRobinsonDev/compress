![Shavian Alphabet](https://www.shavian.info/img/Sampler_Cafe_Majestic_Regular.png)

# Compress

Using the Roman alphabet (also known as the Latin alphabet) isn't very efficient and has led to inconsistencies in the language. 

> *Why is **s** pronounced so many different ways?*
>
>**S**o **/&#115;/**
>**S**ure **/&#643;/**
>Mea**s**ure **/&#658;/**

Furthermore, these inconsistencies and the adoption foreign words into the core English vernacular led to many ghost characters.

>Half of this word is silent: Tho**ugh**

## The Solution? Shavian
![Shavian Alphabet](https://upload.wikimedia.org/wikipedia/commons/thumb/0/0d/Shavian_in_Shavian.svg/200px-Shavian_in_Shavian.svg.png)

The **Shavian alphabet** (also known as the Shaw alphabet) is a constructed alphabet conceived to provide simple, phonemic orthography for the English language to replace the difficulties of conventional spelling using the Latin alphabet. It was posthumously funded by and named after Irish playwright Bernard Shaw.

## How the Sausage is made

- A text file is passed as the second argument of the program.
```bash
compress ./helloworld.txt
```
>```
>𐑣𐑩𐑤𐑩𐑫  𐑢𐑻𐑤𐑛
>```

- This file is parsed, and the text is converted to the IPA (International Phonetic Alphabet) with the cli tool ```espeak-ng```
- This IPA passage is then converted to Shavian

A summary of each step can be seen with the ```--summary``` flag
```bash
compress ./helloworld.txt --summary
```
>```
>###./helloworld.txt######################
>==[ roman ]===============================
>Contents: Hello World
>Characters: 11
>Words: 2
>[-]=======================================
>
>==[ ipa ]=================================
>Contents: həlˈəʊ wˈɜːld
>Characters: 13
>Words: 2
>[-]=======================================
>
>==[ shavian ]=============================
>Contents: 𐑣𐑩𐑤𐑩𐑫 𐑢𐑻𐑤𐑛
>Characters: 10
>Words: 2
>[-]=======================================
>##########################################
>```
>Roman to Shavian saves an entire character here 🤯

## Possible Pronunciation Pitfalls

Shavian is an alphabet designed for the English language; therefore, it doesn't contain all the phonemes that are included in the IPA. This poses some challenges when the IPA is generated from ```espeak-ng```. For instance, **/&#633;/** (a rolled 'r' sound) isn't spoken in American English; instead, it would be pronounced **/r/**. These differences are caught and corrected to the best of my ability but are subject to accent interpretation.

>```rust
>pub fn predictive_fix(ipa: char) -> char {
>    let mut case: HashMap<char, ShavianCharacter> = HashMap::new();
>
>    case.insert('\u{0279}', ShavianCharacter { name: "ROAR", character: '\u{1046E}', phoneme: PhonemeCharacter { ipa: "\u{0072}", examples: ["r","",""] }});
>    case.insert('\u{0250}', ShavianCharacter { name: "ADO", character: '\u{10469}', phoneme: PhonemeCharacter { ipa: "\u{0259}", examples: ["a","o",""] }});
>    case.insert('\u{0069}', ShavianCharacter { name: "EAT", character: '\u{10470}', phoneme: PhonemeCharacter { ipa: "\u{0069}\u{02D0}", examples: ["ee","e",""] }});
>    case.insert('\u{0061}', ShavianCharacter { name: "AH", character: '\u{1046D}', phoneme: PhonemeCharacter { ipa: "\u{0251}\u{02D0}", examples: ["a","",""] }});
>    case.insert('\u{025C}', ShavianCharacter { name: "UP", character: '\u{10473}', phoneme: PhonemeCharacter { ipa: "\u{028C}", examples: ["u","",""] }});
>    
>    match case.get(&ipa) {
>        Some(c) => c.character,
>        None => ipa
>    }
>}
>```
>Convertion is as such:
>- **/&#633;/** => **/&#114;/**
>- **/&#592;/** => **/&#x259;/**
>- **/&#x69;/** => **/&#x69;&#x2D0;/**
>- **/&#x61;/** => **/&#x251;&#x2D0;/**
>- **/&#x25C;/** => **/&#x28C;/**

And with that, I say, goodbye Roman and 𐑣𐑩𐑤𐑩𐑫  𐑖𐑭𐑝𐑾𐑯!
