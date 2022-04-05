fn main() {
    let n = 94;

    let res = generate_nth_fibonacci_number(n);
    println!("The {}th fibonacci number is: {}", n, res);
}

fn generate_nth_fibonacci_number(n: usize) -> usize {
    let mut arr = [0; 95];
    arr[1] = 1;

    for i in 2..n {
        arr[i] = arr[i-1] + arr[i-2];
    }

    arr[n-1]
}
