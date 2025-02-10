// Function `call_me` takes one argument of type `u64`
fn call_me(num: u64) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    call_me(3); // Calls the function with `3` as an argument
}
