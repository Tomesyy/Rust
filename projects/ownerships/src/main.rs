fn main() {
    // let _s = "hello"; // immutable
    let mut s = String::from("hello"); // mutable

    s.push_str(", world!");

    println!("{}", s);

    // Ways variables and data interact. : move

    let _x = 5; // binds 5 to variable x
    let _y = _x; // make a copy of x and bind to y

    // Both variable x and y are of fixed size, hence get pushed onto the stack.

    // The case is different when it comes to the String type.

    let s1 = String::from("hello");
    let s2 = s1; // this a move rather than a copy, because the first variable is invalidated in order to avoid double free error.

    // Rust copied the pointers to the data on the heap for s1
    // and binds it to s2, but it doesn't make a copy of the data in heap
    // itself, because that would be too expensive if the data on the heap were large.

    println!("{}, world!", s2);

    // Ways variables and data interact: Clone

    let s3 = String::from("hello s3");
    let s4 = s3.clone(); // this is a deep-copy; the heap data and pointers in stack is copy and binded to this new variable.

    println!("s3 = {}, s4 = {}", s3, s4);
}