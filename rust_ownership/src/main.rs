fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // ownership moves to s2

    println!("{}", s1); // ❌ error! s1 is no longer valid
}
