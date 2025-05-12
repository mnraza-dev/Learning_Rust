fn main() {
    // print_value();
    let num1: u8 = 100;
    let num2: u8 = 20;
    let result: u8 = add(num1, num2);
    println!("The sum of {} and {} is {}", num1, num2, result);
}
fn print_value() {
    println!("This is a function");
}
fn add(num1: u8, num2: u8) -> u8 {
    return num1 + num2;
}
