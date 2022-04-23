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

    /*
    Rust copied the pointers to the data on the heap for s1
    and binds it to s2, but it doesn't make a copy of the data in heap
    itself, because that would be too expensive if the data on the heap were large.
    */

    println!("{}, world!", s2);




    // Ways variables and data interact: Clone

    let s3 = String::from("hello s3");
    let s4 = s3.clone(); // this is a deep-copy; the heap data and pointers in stack is copy and binded to this new variable.

    println!("s3 = {}, s4 = {}", s3, s4);


    


    // Ownership and Functions.
    let a = String::from("hello");

    takes_ownership(a);

    // println!("{}", a); // throws a compile time error because the varaible has been invalidated by rust

    let z = 5;

    makes_copy(z);

    println!("{}", z);

}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // some_string goes out of scope and the 'drop' is called. The backing memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // some-integer goes out of scope. Nothing special happens.