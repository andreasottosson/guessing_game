use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::time::Instant;

fn main() {
    println!("Guess the number! (Between 1 and 100)");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut guesses: u32 = 0;

    let start = Instant::now();
    
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                guesses += 1;
                println!("Too small!");
            },
            Ordering::Greater => {
                guesses += 1;
                println!("Too big!");
            },
            Ordering::Equal => {
                guesses += 1;
                let duration = start.elapsed();
                println!("You win! You used {guesses} guesses in {:?}.", duration);
                break;
            }
        }    
    }
    
}