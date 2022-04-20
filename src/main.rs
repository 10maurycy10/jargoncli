use std::fs::File;

use std::io::{self, BufRead};

fn jargon_lookup<'a,T: Iterator<Item=&'a str>>(word: &str, mut file: T) -> Option<Vec<&'a str>> {
    let tag = format!(":{}:", word);
    loop {
        match file.next() {
            Some(line) => if line.starts_with(&tag) {
                let mut def: Vec<&'a str> = vec![];
                def.push(line);
                // contine adding lines untill EOF or ':' 
                for line in file {
                    if line.starts_with(":") {
                        break
                    } else {
                      def.push(line)
                    }
                }
                return Some(def)
            }
            None => return None
        }
    }
}

fn create_jargon_index<'a, T: Iterator<Item=&'a str>>(file: T) -> Vec<&'a str> {
    let mut index = vec![];
    for line in file {
        if line.starts_with(":") {
            let line = &line[1..];
            match line.find(":") {
                Some(end_idx) => index.push(&line[..end_idx]),
                None => ()
            }
        }
    }
    index
}

fn main() {

    let mut args = std::env::args();
    let mut jargon_path = "/usr/local/share/jargon".to_owned();
    let mut words_to_define = vec![];
    let mut index = false;

    let bin_name = args.next().unwrap();

    loop {
        match args.next() {
            Some(arg) => match &*arg {
                "define" => words_to_define.push(args.next().expect("define flag requires argument")),
                "file" => jargon_path = args.next().expect("file flag needs argument"),
                "index" => index = true,
                "help" | "--help" | "-h" => {
                    println!("usage: {} [file /path/to/source/file] [define word] [index]", bin_name);
                    println!("\tfile: file path to read jargon from.");
                    println!("\tdefine: print a words entry.");
                    println!("\tindex: outputs a grepable list of words in the file");
                },
                a => {println!("Unknown flag {}!", a); return;}
            },
            None => break
        }
    }

    let jargon_file = File::open(jargon_path).expect("Failed to read jargon file!");
    let jargon = io::BufReader::new(jargon_file).lines();
    let jargon: Vec<_> = jargon.map(|x| x.expect("Error reading file, invalid UTF-8?")).collect();

    for word in words_to_define {
        let def = jargon_lookup(&word, jargon.iter().map(|x| &**x));
        match def {
            Some(def) => for line in def {
                println!("{}", line);
            },
            None => println!("No entry for {}", word)
        }
    }

    if index {
        for line in create_jargon_index(jargon.iter().map(|x| &**x)) {
            println!("{}",line)
        }
    }
}
