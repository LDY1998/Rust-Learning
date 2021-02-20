use std::env;
use std::fs;
use std::process;

mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();



    let res_tuple = parse_args(&args).unwrap_or_else(|err| {
        eprintln!("Problem processing arguments: {}", err);
        process::exit(1);
    });

    let (query, file_name) = &res_tuple;

    println!("Search for : {}", query);
    println!("In File: {}", file_name);

    let contents = fs::read_to_string(file_name)
        .expect("File IO Exception!");

    println!("Lines: {:?}", lib::search(&query, &contents));
}

fn parse_args(args: &[String]) -> Result<(&str, &str), &str> {
    
    if args.len() < 3 {
        return Err("Not enough arguments");
    }
    
    let query = &args[1];
    let file_name = &args[2];
    
    Ok((query, file_name))
}