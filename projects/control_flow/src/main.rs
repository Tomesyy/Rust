fn main() {
    let number = 3;

    // conditions for if/else statemets must be bool

    if number < 5 {
        println!("number less than 5");
    } else {
        println!("number greater or equal to 5");
    }

    let number = 6;

    is_number_divisible(number);

    let result = use_if_in_let_statement();

    println!("The value of result is: {}", result);
}

fn is_number_divisible(number: i32) {
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3 or 2");
    }
}

fn use_if_in_let_statement() -> i32 {
    let condition: bool = false;
    // if and else statements must have the same return types
    let number = if condition { 5 } else { 8 };

    // println!("The value of the number is: {}", number);
    return if number == 5 { number * 2 } else { number / 2 };
}