use std::io;
use rand::Rng;
use std::cmp::Ordering;
 
fn main() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop{
    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    // println!("The secret number is {secret_number}");
    // println!("You guessed: {guess}");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    // if secret_number > guess_u32{
    //     println!("Too small");
    // }
    // else if secret_number < guess_u32{
    //     println!("Too big");
    // }
    // else {
    //     println!("You win");
    // }
    
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

/*Commentaries on the method use 
Explication du : let guess: u32 = guess.trim().parse().expect("Please type a number!");

The trim method on a String instance will eliminate any whitespace at the beginning and end, which we must do to be able to compare the string to the u32, which can only contain numerical data. The user must press enter to satisfy read_line and input their guess, which adds a newline character to the string. For example, if the user types 5 and presses enter, guess looks like this: 5\n. The \n represents “newline”. (On Windows, pressing enter results in a carriage return and a newline, \r\n). The trim method eliminates \n or \r\n, resulting in just 5.
The parse method on strings converts a string to another type. Here, we use it to convert from a string to a number. We need to tell Rust the exact number type we want by using let guess: u32. The colon (:) after guess tells Rust we’ll annotate the variable’s type.

The parse method will only work on characters that can logically be converted into numbers and so can easily cause errors.

Because it might fail, the parse method returns a Result type, much as the read_line method does. We’ll treat this Result the same way by using the expect method again. If parse returns an Err Result variant because it couldn’t create a number from the string, the expect call will crash the game and print the message we give it. If parse can successfully convert the string to a number, it will return the Ok variant of Result, and expect will return the number that we want from the Ok value.





*/