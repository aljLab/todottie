use std::{env, process};
use todottie::todo::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args).unwrap_or_else(|err| {
        println!("Errorn when parsing the arguments: {err}");
        process::exit(1);
    });

    println!("Config: {}, {}", config.command, config.arg);

    run(config);
}

fn parse_config(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
        return Err("Not enough arguments!")
    }

    let command = args[1].clone();
    let arg = args[2].clone();

    Ok(Config { command, arg })
}

