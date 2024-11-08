use rand::{thread_rng, Rng};
use std::io;

fn main() {
    println!("Guessing has started!");

    let secret_number: i32 = thread_rng().gen_range(1..=100);

    loop {
        println!("Kindly enter your guess: ");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("The input is invalid");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Greater => {
                println!("Guess is greater")
            },
            std::cmp::Ordering::Less => {
                println!("Guess is lesser")
            },
            std::cmp::Ordering::Equal => {
                println!("You won");
                break;
            },
        };
    }
}
