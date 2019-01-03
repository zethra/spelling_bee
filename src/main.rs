use std::io::Read;
use std::fs::File;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage <file> <letters>.  First letter in list is the required letter");
        return
    }
    let file_name = args[1].clone();
    let letters = args[2].clone();
    let mut allowed: Vec<char> = Vec::new();
    let required = letters.chars().nth(0).unwrap();
    for letter in letters.chars() {
        allowed.push(letter.clone());
    }
    let mut file = File::open(&file_name).expect("Failed to open words file");
    let mut buf = String::with_capacity(4_000_000);
    file.read_to_string(&mut buf).expect("Failed to read file to buf");
    for word in buf.split("\n") {
        if word.len() > 4 && word.contains(required) {
            let mut good = true;
            for letter in word.chars() {
                if !allowed.contains(&letter) {
                    good = false;
                    break;
                }
            }
            if good {
                println!("{}", word);
            }
        }
    }
}
