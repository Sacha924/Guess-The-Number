use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("The secret number is {secret_number}");
    println!("You guessed: {guess}");

    let guess_u32 : u32 = guess.trim().parse().expect("Please type a number!");

    if secret_number > guess_u32{
        println!("Too big");
    }
    else if secret_number < guess_u32{
        println!("Too small");
    }
    else {
        println!("You win");
    }
}
