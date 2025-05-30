use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("Guess the number between 1 and 100:");
    println!("Number: ");


    let rand_number = rand::thread_rng()
        .gen_range(1..=100);

    loop{
        let mut guess = String::new();
        
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed do read");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {}", guess);
    
        match guess.cmp(&rand_number){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You've guessed");
                break;
            }
        }

        println!("\n");
    }

    
}
