fn main() {
    let num:u8 = 56;

    // num = 100; // This will cause an error because num is immutable
    // To fix this, we can declare num as mutable

    let mut num:u8 = 56;
    num = 100; // Now this is valid
    println!("The value stored in num is {}", num);

    // Rust does not allow you to change the type of a variable
    // We can also use the `let` keyword to create a new variable with the same name
    // but a different type

    let num:u16 = 1000; // This is valid because num is now a new variable

    println!("The value stored in num is {}", num);
}
