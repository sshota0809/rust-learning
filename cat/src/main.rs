use std::fs::File;
use std::io;
use std::io::BufRead;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Specify a file path as an argument");
        process::exit(1);
    }
    let filename = &args[1];

    let f = File::open(filename);
    let file;
    match f {
        Ok(f) => file = f,
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    }
    let f = io::BufReader::new(file);

    for ll in f.lines() {
        println!("{}", ll.unwrap())
    }
}
