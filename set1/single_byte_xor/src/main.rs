use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

extern crate hex;
fn is_alphabetic(s: &String) -> bool {
    for c in s.chars() {
        if (c as u8) < 32 || (c as u8) >122 {
            return false;
        }
    }
    true
}
fn decode(ciphertext: Vec<u8>) -> Vec<String> {
    let mut text: Vec<String> = Vec::new();
    for i in 0..255 {
        let mut plain = String::new();
        for c in ciphertext.iter() {
            plain.push((i^c) as char)        
        } 
        text.push(plain);
    }
    return text;
}

fn checker(plains: Vec<String>) -> Vec<(String)>{
    let mut ans: HashMap<String,f32> = HashMap::new();
    let LETTER_FREQUENCY: HashMap<&str,f32> = 
        [("e",0.1202),
        ("t",0.091),
        ("a",0.0812),
        ("o",0.0768),
        ("i",0.0731),
        ("n",0.0695),
        ("s",0.0628),
        ("r",0.0602),
        ("h",0.0592),
        ("d",0.0432),
        ("l",0.0398),
        ("u",0.0288),
        ("c",0.0271),
        ("m",0.0261),
        ("f",0.0230),
        ("y",0.0211),
        ("w",0.0209),
        ("g",0.0203),
        ("p",0.0182),
        ("b",0.0149),
        ("v",0.0111),
        ("k",0.0069),
        ("x",0.0017),
        ("q",0.0011),
        ("j",0.0010),
        ("z",0.0007)]
        .iter().cloned().collect();
    let texts: Vec<String> = plains
        .into_iter()
        .filter(|x| is_alphabetic(x)) 
        .collect();
   for s in texts.iter() {
        let o = s.to_lowercase();
        let mut frq_diff = 0f32;
        for c in LETTER_FREQUENCY.keys() {
            let num = o.matches(c).count() as f32;
            frq_diff += (num/o.len() as f32) - LETTER_FREQUENCY[c];
        }
        ans.insert(s.to_string(), frq_diff);
   }
    let mut texts: Vec<(&String,&f32)> = ans.iter().collect();
    texts.sort_by(|a,b| a.1.partial_cmp(b.1).unwrap());
    let mut plain: Vec<String> = Vec::new();
    for (i,_) in texts {
        plain.push(i.to_string());
    } 
    return plain;
}
fn main() {
    // let ciphertext = match hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736") { //For challenge 3
    //     Ok(c) =>c,
    //     Err(e) => panic!("Error: {}",e)
    // };
    
    let f = File::open("4.txt").unwrap();
    let f = BufReader::new(f);
    for line in f.lines() {
        let ciphertext = match hex::decode(line.unwrap()) { //For challenge 3
            Ok(c) =>c,
            Err(e) => panic!("Error: {}",e)
        };
        let plains = decode(ciphertext);
        let plaintext = checker(plains);
        for i in plaintext {
            println!("{}",i);
        }
    }

    // let plains = decode(ciphertext);
    // 
    // let plaintext = checker(plains);
    // for i in plaintext {
    //     println!("{}",i);
    // }
}
