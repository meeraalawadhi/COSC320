fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // Get a slice from the array `a` (elements 2, 3, and 4)
        let nice_slice = &a[1..4]; // Slicing from index 1 to 3 (inclusive of 1, exclusive of 4)

        assert_eq!([2, 3, 4], nice_slice);
    }
}
