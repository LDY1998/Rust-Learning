use std::io;
use std::io::{Write};
use std::fs::File;
use std::fs;



pub struct Repl;


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
                    self.load(cmd[1]);
                }
                _ => { continue; },
            }
        }
    }

    fn load(&self, file: &str) {

        // let f = File::open(format!("./script/{}", file)).expect("File does not exist");

        let path = &format!("./example/{}", file);

        let f = match File::open(path) {
            Ok(_) => (),
            Err(error) => eprintln!("{}", error),
        };

        let content = fs::read_to_string(path).expect("Cannot read the file");

        println!("File content: {}", content);
    }
}