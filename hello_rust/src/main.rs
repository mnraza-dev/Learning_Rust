fn main() {
    let num: u8 = 56;

    // num = 100; // This will cause an error because num is immutable
    // To fix this, we can declare num as mutable

    let mut num: u8 = 56;
    num = 100; // Now this is valid
    println!("The value stored in num is {}", num);

    // Rust does not allow you to change the type of a variable
    // We can also use the `let` keyword to create a new variable with the same name
    // but a different type

    let num: u16 = 1000; // This is valid because num is now a new variable
    println!("The value stored in num is {}", num);


    //  String Literals - Fixed length string
    // String - Dynamic length string
    // String literals are immutable and stored in the read-only memory
    // String objects are mutable and stored in the heap
    // String literals are created using double quotes
    // String objects are created using the String::from() method

    let string_literal: &str = "Hello, there!"; // This is a string literal
    println!("The value stored in string_literal is {}", string_literal);

    let mut string = String::from("Hello, there!"); // This is a String object
    string.push_str(" How are you?"); // This is valid because string is mutable

    println!("The value stored in string is {}", string);

    // Rust has a special type called `Option` that can be used to represent a value that may or may not be present
    let optional_value: Option<u8> = Some(56);
    println!("The value stored in optional_value is {:?}", optional_value);

    let optional_value: Option<u8> = None;
    println!("The value stored in optional_value is {:?}", optional_value);
}
