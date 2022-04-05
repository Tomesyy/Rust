fn main() {
    another_function(5);

    print_labeled_measurement(34, 'h');

    let value = five();

    println!("The value is: {}", value);

    let plus_one_value = plus_one(value);

    println!("The value of plusone is: {}", plus_one_value);
}

// rust doesn't care where you define your functions,
// only that they are defined somewhere in the program

// in function declaration, you must declare the type for each parameter

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

// function with return values
// return values do not end with semi-colon

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    // return x + 1;
    // or
    x + 1
}