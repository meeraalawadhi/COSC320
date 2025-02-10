// Returns how much ice cream is left based on the hour of the day.
fn maybe_icecream(hour_of_day: u16) -> Option<u16> {
    if hour_of_day > 23 {
        None // Invalid hours return None
    } else if hour_of_day < 22 {
        Some(5) // Before 22:00, 5 scoops are left
    } else {
        Some(0) // At 22:00 and later, all ice cream is gone
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_value() {
        // Fix: Extract the value inside `Option<u16>` using `.unwrap()`
        let icecreams = maybe_icecream(12).unwrap();

        assert_eq!(icecreams, 5); // Don't change this line.
    }

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(0), Some(5));
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(18), Some(5));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(24), None);
        assert_eq!(maybe_icecream(25), None);
    }
}
