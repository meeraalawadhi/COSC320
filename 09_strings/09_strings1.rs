// Fix: Convert the &str to a String
fn current_favorite_color() -> String {
    "blue".to_string() // or String::from("blue")
}

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {answer}");
}
