fn main() {
    learn_loop();

    learn_labelled_loops();

    learn_return_loop_values();

    learn_while_loops();

    learn_for_loops();
}

fn learn_loop() {
    let mut count: u8 = 0;
    loop {
        if count >= 5 {
            break;
        }
        println!("loop again!");
        count += 1;
    }
}

fn learn_labelled_loops() {
    let mut count = 0;

    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    println!("End count = {}", count);
}

fn learn_return_loop_values() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn learn_while_loops() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("Take Off!!!");
}

fn learn_for_loops() {
    let a: [i32; 5] = [10, 20, 30, 40, 50];

    for number in a {
        println!("the value is: {}", number);
    }

    for number in (1..=3).rev() {
        println!("{}!", number);
    }
    
    println!("Take Off!!!");
}