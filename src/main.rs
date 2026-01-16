use std::cmp::Ordering;
use std::io;
use rand::Rng;


fn game(high_score: u32) -> u32 {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guess_count: u32 = 1;

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
    let mut new_high_score: u32 = high_score;
    if high_score > guess_count {
        new_high_score = guess_count;
    }
    new_high_score

}

fn main () {
    let mut high_score: u32 = 101;
    let mut repeater = true;
    while repeater {
        high_score = game(high_score);

        println!("Would you like to play again? (yes or no)");
        let mut user_repeating = String::new();

        io::stdin()
            .read_line(&mut user_repeating)
            .expect("Failed to read line");

        let repeat_answer = user_repeating.chars().next().unwrap().to_ascii_lowercase();

        if 'y' != repeat_answer {
            repeater = false;
        }

    }
    let score_string: String = high_score.to_string();
    println!("Your best score is {score_string}!");
    println!("Play again soon!");
}
