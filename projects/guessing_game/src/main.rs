use std::io;

fn main() {
    println!("guess the number");

    println!("Please enter your guessed number:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    println!("You guessed: {}", guess);
}
