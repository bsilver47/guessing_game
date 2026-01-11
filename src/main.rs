use std::cmp::Ordering;
use std::io;
use rand::Rng;


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guess_count: i32 = 1;

    loop {
        println!("Please input your guess:");
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
                println!("Too small");
                guess_count += 1;
            }
            Ordering::Greater => {
                println!("Too big");
                guess_count += 1;
            }
            Ordering::Equal => {
                println!("You win!");
                println!("You got it in {guess_count} guesses!");
                break;
            }
        }
    }
}
