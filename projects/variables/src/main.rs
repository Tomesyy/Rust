fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //CONSTANTS

    //just like variables initially, constants are immutable.
    //unlike variables, constants are always immutable and can never be made mutuable
    //use uppercase seperate with an underscore when naming constants
    const THREE_HOURS_IN_SECONDS : u32 = 60 * 60 * 3;
    println!("Three hours in seconds is {} seconds", THREE_HOURS_IN_SECONDS);

    // SHADOWING
    // means the most recent value in the scope is what the program sees
    // when the variable is used.
    

    let x = 5;

    let x = x + 1;
    
    {
        let x = x + 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x in the outer scope is: {}", x);

    // you can change the type but reuse the same name in shadowing

    let spaces = "   ";
    let spaces = spaces.len();

    // DATA-TYPES

    let guess : u32 = "42".parse().expect("Not a number");



    // Scalar Types
    // - integers, floating-points, booleans, and characters.

    // Integer Types
    // signed varaints of integer types can store from -2^(n-1) - 2^(n-1)-1
    // unsigned variants of integer types can store from 0 - 2^(n-1)-1
    // where n is the number of bits.
}
