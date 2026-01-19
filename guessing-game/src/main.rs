use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number from 1 to 10!");
    let secret_number = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // mutable

        io::stdin()
            .read_line(&mut guess) // & => indicates that this argument is a reference
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
