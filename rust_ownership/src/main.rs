fn main() {
    let x: u8 = 5;
    let s1 = String::from("hello");
    let s2 = s1; // ownership moves to s2

    // println!("{}", s1); // ❌ error! s1 is no longer valid
    println!("{}", s2); // ✅ s2 is valid

    process_interger(x);
}

fn process_interger(x: u8) {
    println!("{}", x);
}
