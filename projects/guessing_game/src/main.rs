use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guess the number");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please enter your guessed number:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
        
        
        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! \n"),
            Ordering::Greater => println!("Too big! \n"),
            Ordering::Equal => {
                println!("Yay, congratulations!! You win!");
                break;
            },
        }
    }
    
}
