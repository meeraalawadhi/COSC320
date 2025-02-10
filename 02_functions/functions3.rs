fn call_me(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    // Fix: Provide an argument to `call_me`
    call_me(5); // Example: Passing 5 as an argument
}
