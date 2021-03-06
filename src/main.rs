mod amps;
mod cli;

mod engine;

fn main() -> Result<(), std::io::Error> {
    cli::main_loop();

    Ok(())
}

/*
fn main_a() -> Result<(), std::io::Error> {
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
*/
