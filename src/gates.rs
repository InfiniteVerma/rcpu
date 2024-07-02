pub fn and_gate(x: bool, y: bool) -> bool {
    x && y
}

pub fn or_gate(x: bool, y: bool) -> bool {
    x || y
}

pub fn not_gate(x: bool) -> bool {
    !x
}

pub fn nand_gate(x: bool, y: bool) -> bool {
    not_gate(and_gate(x, y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_and() {
        let res = and_gate(false, false);
        assert_eq!(res, false);

        let res = and_gate(true, false);
        assert_eq!(res, false);

        let res = and_gate(false, true);
        assert_eq!(res, false);

        let res = and_gate(true, true);
        assert_eq!(res, true);
    }
}
