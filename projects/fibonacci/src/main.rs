use std::io;

fn main() {

    loop {
        println!("Please input a number: ");

        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("failed to input");
        
        let number: u8 = match number.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("invalid. try inputting another number\n");
                continue
            }
        };

        let res: u32 = fib_recursive(number);
        println!("The {}th fibonacci number is {}\n", number, res);
    }
}

fn _fib_iterative(n: usize) -> usize {
    let mut arr = [0; 95];
    arr[1] = 1;

    for i in 2..n {
        arr[i] = arr[i-1] + arr[i-2];
    }

    arr[n-1]
}

fn fib_recursive(n: u8) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    return fib_recursive(n-1) + fib_recursive(n-2)
}
