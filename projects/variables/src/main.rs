fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //CONSTANTS
    
    //just like variables initially, constants are immutable.
    //unlike variables, constants are always immutable and can never be made mutuable

    const THREE_HOURS_IN_SECONDS : u32 = 60 * 60 * 3;
    println!("Three hours in seconds is {} seconds", THREE_HOURS_IN_SECONDS);
}
