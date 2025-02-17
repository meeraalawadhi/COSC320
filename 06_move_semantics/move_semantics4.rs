fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        
        // First mutable borrow and modification
        {
            let y = &mut x;
            y.push(42);
        } // `y` goes out of scope here

        // Now a new mutable reference can be created
        let z = &mut x;
        z.push(13);

        assert_eq!(x, [42, 13]);
    }
}
