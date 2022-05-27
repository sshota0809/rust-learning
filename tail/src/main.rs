use clap::Parser;
use easy_reader::EasyReader;
use std::fs::File;
use std::process;

#[derive(Parser, Default, Debug)]
struct Arguments {
    /// Name of the file
    file_name: String,
    #[clap(short, long, default_value_t = 10)]
    /// The number of lines from the end to read
    number: usize,
}

fn main() {
    let args: Arguments = Arguments::parse();

    let f = File::open(&args.file_name);
    let file;
    match f {
        Ok(f) => file = f,
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    }

    let mut f = EasyReader::new(file).unwrap();
    let rf = f.eof();

    let mut pl: Vec<String> = vec![];
    for _ in 0..args.number {
        match rf.prev_line().unwrap() {
            Some(ll) => pl.push(ll),
            None => break,
        }
    }
    for ll in pl.iter().rev() {
        println!("{}", ll);
    }
}
