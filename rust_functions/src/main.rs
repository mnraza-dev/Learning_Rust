fn main() {
    print_value();
    sum_value(10, 20);
}

fn print_value() {
    println!("This is a function");
}
fn sum_value(num1:u8, num2:u8){
    let item = num1 + num2;
    println!("the sum of {} and {} is - {}",num1, num2, item);

}