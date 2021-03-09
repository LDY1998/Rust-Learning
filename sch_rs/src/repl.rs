use std::io;
use std::io::{Write};
use std::fs;
use crate::interpreter::{lex::Lexer, parser::Parser};


pub struct Repl {
}


impl Repl {
    pub fn run(&self) {
        
        let mut cmd = String::new();

        println!("start the repl");

        loop {

            cmd.clear();
            
            print!("scheme_rs> ");

            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut cmd).unwrap();


            let cmd: Vec<&str> = cmd.trim().split(" ").collect();

            assert_eq!(cmd.len()<=2, true);
            
            match cmd[0] {
                "quit" => {
                    println!("quitting the sch_rs");
                    break;
                },
                "load" => {
                    println!("load command: {}", cmd[1]);
                    match self.load(cmd[1]) {
                        Ok(s) => {
                            println!("Tokenize result: {:?}", self.interp(&s))
                        },
                        Err(e) => eprintln!("Error in loading exmaple {}: {}", cmd[1], e),
                    };
                }
                _ => { continue; },
            }
        }
    }

    fn load(&self, file: &str) -> Result<String, io::Error> {

        // let f = File::open(format!("./script/{}", file)).expect("File does not exist");

        let path = &format!("./example/{}", file);

        fs::read_to_string(path)
    }

    fn interp(&self, input: &String) {
        match Lexer::lex(input) {
            Ok(tokens) => {
                let nodes = Parser::parse(&tokens);
            }
            _ => panic!(format!("Error in lexing input: {}", input)),
        }
    }
}