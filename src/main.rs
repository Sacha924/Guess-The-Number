use std::io;
use rand::Rng;
use std::cmp::Ordering;
 
fn main() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop{
    println!("Please input your guess");

    let mut guess = String::new();  //this line has created a mutable variable that is currently bound to a new, empty instance of a String.

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

io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

we call the stdin function from the io module, which will allow us to handle user input. If we hadn’t imported the io library with use std::io at the beginning of the program, we could still use the function by writing this function call as std::io::stdin. The stdin function returns an instance of std::io::Stdin, which is a type that represents a handle to the standard input for your terminal.

Next, the line .read_line(&mut guess) calls the read_line method on the standard input handle to get input from the user. We’re also passing &mut guess as the argument to read_line to tell it what string to store the user input in. The full job of read_line is to take whatever the user types into standard input and append that into a string (without overwriting its contents), so we therefore pass that string as an argument. The string argument needs to be mutable so the method can change the string’s content.

The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. References are a complex feature, and one of Rust’s major advantages is how safe and easy it is to use references. You don’t need to know a lot of those details to finish this program. For now, all you need to know is that like variables, references are immutable by default. Hence, you need to write &mut guess rather than &guess to make it mutable







Explication du : let guess: u32 = guess.trim().parse().expect("Please type a number!");

The trim method on a String instance will eliminate any whitespace at the beginning and end, which we must do to be able to compare the string to the u32, which can only contain numerical data. The user must press enter to satisfy read_line and input their guess, which adds a newline character to the string. For example, if the user types 5 and presses enter, guess looks like this: 5\n. The \n represents “newline”. (On Windows, pressing enter results in a carriage return and a newline, \r\n). The trim method eliminates \n or \r\n, resulting in just 5.
The parse method on strings converts a string to another type. Here, we use it to convert from a string to a number. We need to tell Rust the exact number type we want by using let guess: u32. The colon (:) after guess tells Rust we’ll annotate the variable’s type.

The parse method will only work on characters that can logically be converted into numbers and so can easily cause errors.

Because it might fail, the parse method returns a Result type, much as the read_line method does. We’ll treat this Result the same way by using the expect method again. If parse returns an Err Result variant because it couldn’t create a number from the string, the expect call will crash the game and print the message we give it. If parse can successfully convert the string to a number, it will return the Ok variant of Result, and expect will return the number that we want from the Ok value.




Explication du Ordering:
The Ordering type is another enum and has the variants Less, Greater, and Equal. These are the three outcomes that are possible when you compare two values.
We use this, and also the comp Method. The cmp method compares two values and can be called on anything that can be compared.
Here it’s comparing the guess to the secret_number. Then it returns a variant of the Ordering enum we brought into scope with the use statement. We use a match expression to decide what to do next based on which variant of Ordering was returned from the call to cmp with the values in guess and secret_number.
*/