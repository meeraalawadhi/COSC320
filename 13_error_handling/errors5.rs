use std::error::Error;
use std::fmt;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

// Implement `Display` for `CreationError` so it can be used as an `Error`
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl Error for CreationError {}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

// Fix: Change `main` to return `Result<(), Box<dyn Error>>`
fn main() -> Result<(), Box<dyn Error>> {
    let pretend_user_input = "42";

    // `parse::<i64>()` may return a `ParseIntError`, which `?` propagates
    let x: i64 = pretend_user_input.parse()?;

    // `PositiveNonzeroInteger::new(x)?` may return a `CreationError`, which `?` propagates
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);

    Ok(())
}
