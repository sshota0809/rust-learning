use std::fs::File;
use std::io;
use std::io::BufRead;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Specify character string and file path as arguments");
        process::exit(1);
    }
    let grep_string = &args[1];
    let filename = &args[2];

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
        let line = ll.unwrap();
        if line.contains(grep_string) {
            println!("{}", line);
        }
    }
}
