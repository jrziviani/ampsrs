use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::ErrorKind;

mod token_types;
mod token;
mod metadata;
mod scan;
mod context;
mod generator;

use scan::scanner;
use generator::parsing;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() != 2 {
        eprintln!("usage: ./{} <filename>", args[0]);
    }

    let file = File::open(&args[1]);
    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                panic!("{} not found", args[1])
            },
            other_error => {
                panic!("{:?}", other_error)
            }
        }
    };

    let mut buf_reader = BufReader::new(file);

    println!("Scaning template...");
    let info = scanner::scan(&mut buf_reader);
    parsing::parse(&info);
    //for i in info {
    //    println!("{:#?}", i);
    //}

    Ok(())
}
