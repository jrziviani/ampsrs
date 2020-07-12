use rustyline::Editor;

use crate::amps::Amps;

pub fn main_loop() {
    println!("Welcome to Amps cli.");

    let mut repl = Editor::<()>::new();
    if repl.load_history("amps_history.txt").is_err() {

    }

    let mut amps = Amps::new();
    loop {
        let readline = repl.readline("amps> ");
        match readline {
            Ok(line) => {
                if line == "quit" {
                    break;
                }

                if line == "help" {
                    help();
                    continue;
                }

                if line == "show" {
                    show_template(&amps);
                    repl.add_history_entry(line.as_str());
                    continue;
                }

                repl.add_history_entry(line.as_str());
                parse_command(line.as_str(), &mut amps);
            },
            Err(err) => {
                println!("Quitting: {:?}", err);
                break;
            }
        }
    }

    repl.save_history("amps_history.txt").unwrap();
}

fn parse_command(command: &str, amps: &mut Amps) {
    let command_with_args: Vec<&str> = command.split(' ').collect();

    match command_with_args.first().unwrap().as_ref() {
        "load"    => load(&command_with_args[1..], amps),
        "render"  => render(&command_with_args[1..], amps),
        _         => println!("invalid command: {}", command),
    }
}

fn load(args: &[&str], amps: &mut Amps) {
    if args.is_empty() {
        println!("load [--file filename | template]");
        return;
    }

    if args[0] == "--file" {
        if args.len() == 1 {
            println!("missing filename");
            return;
        }

        // cannot have files with blank spaces
        let filename: String = args[1..].iter().map(|s| s.to_string() + " ").collect();
        amps.load_template_from_file(&filename);
    }
    else {
        let template: String = args[0..].iter().map(|s| s.to_string() + " ").collect();
        amps.load_template(template);
    }
}

fn show_template(amps: &Amps) {
    match amps.get_template() {
        Some(tpl) => println!("{}", tpl),
        None => println!("no template loaded"),
    }
}

fn render(args: &[&str], amps: &mut Amps) {
    let mut i : usize = 0;
    for e in amps.get_errors().iter() {
        print!("{:?}", e);
        i += 1;
    }

    if i == 0 {
        amps.render();
    }
}

fn help() {
    println!("Commands: ");
    println!(" - help:\tPrint this help");
    println!(" - quit:\tExit Amps cli");
}
