fn main() {
    let x: u8 = 5;
    let s1 = String::from("hello there..!");
    // let s2 = s1; // ownership moves to s2

    // println!("{}", s1); // ❌ error! s1 is no longer valid
    println!("{}", s1); // ✅ s2 is valid

    process_interger(x);
    process_string(s1); 

    // println!("{}", s1); // ❌ error! s1 is no longer valid
}

fn process_interger(x: u8) {
    println!("{}", x);
}
fn process_string(s: String) {
    println!("{}", s);
}

/*
NOTE: Ownership
🎯 The Three Rules of Ownership

1. Each value has a single owner.

2. A value can only have one owner at a time.

3. When the owner goes out of scope, the value is dropped.

 */
