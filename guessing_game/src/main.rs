use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    println!("please input your guess");

    let secret_number = rand::rng().random_range(1..=100);
    // println!("The secret number is: {secret_number}");

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
        .expect("failed to read line");
        
        println!("you guessed {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, 
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
