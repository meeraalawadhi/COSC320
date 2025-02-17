

fn main() {
    // `u8` and `i8` can both be converted to `i16`.
    let mut numbers: Vec<i16> = Vec::new();
    //             ^^^^^^^^^^ added

    // Don't change the lines below.
    let n1: u8 = 42;
    numbers.push(n1.into());
    let n2: i8 = -1;
    numbers.push(n2.into());

    println!("{numbers:?}");
}
