// tests/test_gates.rs

// Import necessary modules and functions for testing
mod gates {
    pub(crate) use crate::gates::and_gate;
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_and_gate() {
        assert_eq!(and_gate(true, true), true);
        assert_eq!(and_gate(true, false), false);
        assert_eq!(and_gate(false, true), false);
        assert_eq!(and_gate(false, false), false);
    }
}

