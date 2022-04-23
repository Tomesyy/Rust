fn main() {
    // let _s = "hello"; // immutable
    let mut s = String::from("hello"); // mutable

    s.push_str(", world!");

    println!("{}", s);
}