#![allow(clippy::ptr_arg)]

// Borrows instead of taking ownership.
fn get_char(data: &str) -> char {  // Changed `&String` to `&str`
    data.chars().last().unwrap()
}

// Takes ownership and returns the uppercase string.
fn string_uppercase(data: String) -> String {  // Now returns a String
    data.to_uppercase() 
}

fn main() {
    let data = "Rust is great!".to_string();

    let last_char = get_char(&data);
    println!("Last character: {}", last_char);

    let uppercase_data = string_uppercase(data);
    println!("{uppercase_data}");
}
