fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    // Solution 1: Move `string2` outside the inner block so it is not dropped before `result` is used.
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let result;
    {
        result = longest(&string1, &string2);
    }
    println!("The longest string is '{result}'");

    // ========================================================================

    // Solution 2: Move the print statement inside the inner block so it is executed before `string2` is dropped.
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(&string1, &string2);
        println!("The longest string is '{result}'");
    } // `string2` is dropped here, but `result` was already printed.
}
