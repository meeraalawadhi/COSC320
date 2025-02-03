fn main() {
    let is_morning = true;
    if is_morning {
        println!("Good morning!");
    }

    // Define `is_evening` as the opposite of `is_morning`
    let is_evening: bool = !is_morning;

    if is_evening {
        println!("Good evening!");
    }
}
