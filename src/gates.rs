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

pub fn xor_gate(x: bool, y: bool) -> bool {
    let out1 = nand_gate(x, x);
    let out2 = nand_gate(y, y);
    let out3 = nand_gate(out1, y);
    let out4 = nand_gate(out2, x);
    nand_gate(out3, out4)
}

pub fn mux(x: bool, y: bool, sel: bool) -> bool {
    let out2 = not_gate(sel);
    let out3 = and_gate(x, out2);
    let out4 = and_gate(y, sel);
    or_gate(out3, out4)
}

pub fn dmux(x: bool, sel: bool) -> (bool, bool) {
    let out1 = not_gate(sel);
    (and_gate(x, out1), and_gate(x, sel))
}

pub fn not16_gate(x: u16) -> u16 {
    let mut result = 0;

    for i in 0..16 {
        let bit = (x >> i) & 1 != 0;
        let res = not_gate(bit);
        result |= (res as u16) << i;
    }

    result
}

pub fn and16_gate(x: u16, y: u16) -> u16 {
    let mut result = 0;

    for i in 0..16 {
        let x_bit = (x >> i) & 1 != 0;
        let y_bit = (y >> i) & 1 != 0;
        let res = and_gate(x_bit, y_bit);
        result |= (res as u16) << i;
    }

    result
}

pub fn or16_gate(x: u16, y: u16) -> u16 {
    let mut result = 0;

    for i in 0..16 {
        let x_bit = (x >> i) & 1 != 0;
        let y_bit = (y >> i) & 1 != 0;
        let res = or_gate(x_bit, y_bit);
        result |= (res as u16) << i;
    }

    result
}

pub fn mux16_gate(x: u16, y: u16, sel: bool) -> u16 {
    let mut result = 0;

    for i in 0..16 {
        let x_bit = (x >> i) & 1 != 0;
        let y_bit = (y >> i) & 1 != 0;
        let res = mux(x_bit, y_bit, sel);
        result |= (res as u16) << i;
    }
    result
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

    #[test]
    fn test_xor() {
        let res = xor_gate(false, false);
        assert_eq!(res, false);

        let res = xor_gate(true, false);
        assert_eq!(res, true);

        let res = xor_gate(false, true);
        assert_eq!(res, true);

        let res = xor_gate(true, true);
        assert_eq!(res, false);
    }

    #[test]
    fn test_mux() {
        let res = mux(false, false, false);
        assert_eq!(res, false);

        let res = mux(false, true, false);
        assert_eq!(res, false);

        let res = mux(false, true, true);
        assert_eq!(res, true);
    }

    #[test]
    fn test_dmux() {
        let res = dmux(false, false);
        assert_eq!(res, (false, false));

        let res = dmux(false, true);
        assert_eq!(res, (false, false));

        let res = dmux(true, false);
        assert_eq!(res, (true, false));

        let res = dmux(true, true);
        assert_eq!(res, (false, true));
    }

    #[test]
    fn test_not16() {
        let inp: u16 = 0b0000000000000001;
        let res = not16_gate(inp);
        assert_eq!(res, !inp);
    }

    #[test]
    fn test_and16() {
        let x: u16 = 0b0000000000000001;
        let y: u16 = 0b0000000000000011;
        let res = and16_gate(x, y);
        assert_eq!(res, x & y);
    }

    #[test]
    fn test_or16() {
        let x: u16 = 0b0000000000000001;
        let y: u16 = 0b0000000000000011;
        let res = or16_gate(x, y);
        assert_eq!(res, x | y);
    }

    #[test]
    fn test_mux16() {
        let x: u16 = 0b1001100001110110;
        let y: u16 = 0b0000000000000000;
        let sel = false;
        let res = mux16_gate(x, y, sel);
        assert_eq!(res, 0b1001100001110110);
    }
}
