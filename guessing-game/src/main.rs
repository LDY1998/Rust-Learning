use std::io;
use rand::Rng;
use std::cmp::Ordering;




fn main() {
    
    println!("Result of fib: {} ", fib(10, 1, 1));
}

fn fib (n: u32, a: u32, b: u32) -> u32{

    if n == 1 {
        return b;
    } else if n == 2 {
        return b;
    } else {
        return fib(n-1, b, a+b);
    }
}
fn guess_game() {
  println!("Guessing game!");



    let secret = rand::thread_rng().gen_range(1, 101); 


    loop {
        println!("Please input your number: ");
        let mut guess = String::new();
        io::stdin() 
            .read_line(&mut guess)
            .expect("Faill to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

    
        println!("You guess: {}", guess);



        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }   
   }






}
