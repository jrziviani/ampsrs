use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::ErrorKind;
use std::io::prelude::*;

mod token_types;
mod token;
mod scan;
mod metadata;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    //ig args.len() != 2
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

    let buf_reader = BufReader::new(file);

    println!("Scaning template...");
    let mut scanner = scan::Scan::new();
    for line in buf_reader.lines() {
        //scanner.scan(line.unwrap());
        scanner.parse_block(line.unwrap());
    }
    scanner.print_block();


    //while !iter.peek().is_none() && iter.peek() != Some(&'\n') {
    //    print!("{:?}", iter.peek().);
    //    iter.next();
    //}

    Ok(())
}
