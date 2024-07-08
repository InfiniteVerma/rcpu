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

pub fn dmux_gate(x: bool, sel: bool) -> (bool, bool) {
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

pub fn or8way_gate(x: u8) -> bool {
    let mut arr: [bool; 4] = [false; 4];

    let mut index = 0;
    for _ in 0..4 {
        let x_bit_1 = (x >> index) & 1 != 0;
        let x_bit_2 = (x >> (index + 1)) & 1 != 0;

        let x_res = or_gate(x_bit_1, x_bit_2);

        arr[index] = x_res;
        index += 1;
    }

    let mut arr2: [bool; 2] = [false; 2];

    arr2[0] = or_gate(arr[0], arr[1]);
    arr2[1] = or_gate(arr[2], arr[3]);

    or_gate(arr2[0], arr2[1])
}

pub fn mux4way16_gate(a: u16, b: u16, c: u16, d: u16, sel: u8) -> u16 {
    let sel_1 = sel & 1;
    let sel_2 = (sel >> 1) & 1;
    let out_1 = mux16_gate(a, b, sel_1 != 0);
    let out_2 = mux16_gate(c, d, sel_1 != 0);
    mux16_gate(out_1, out_2, sel_2 != 0)
}

pub fn mux8way16_gate(
    a: u16,
    b: u16,
    c: u16,
    d: u16,
    e: u16,
    f: u16,
    g: u16,
    h: u16,
    sel: u8,
) -> u16 {
    let sel_1 = sel & 1;
    let sel_2 = (sel >> 1) & 1;
    let sel_3 = (sel >> 2) & 1;

    let out_1 = mux16_gate(a, b, sel_1 != 0);
    let out_2 = mux16_gate(c, d, sel_1 != 0);
    let out_3 = mux16_gate(e, f, sel_1 != 0);
    let out_4 = mux16_gate(g, h, sel_1 != 0);

    let out_5 = mux16_gate(out_1, out_2, sel_2 != 0);
    let out_6 = mux16_gate(out_3, out_4, sel_2 != 0);

    mux16_gate(out_5, out_6, sel_3 != 0)
}

pub fn dmux4way_gate(inp: bool, sel: u8) -> (bool, bool, bool, bool) {
    let sel_1 = sel & 1;
    let sel_2 = (sel >> 1) & 1;

    let (out_1, out_2) = dmux_gate(inp, sel_1 != 0);
    let (a, b) = dmux_gate(out_1, sel_2 != 0);
    let (c, d) = dmux_gate(out_2, sel_2 != 0);

    (a, b, c, d)
}

pub fn dmux8way_gate(inp: bool, sel: u8) -> u8 {
    let sel_1 = sel & 1;
    let sel_2 = (sel >> 1) & 1;
    let sel_3 = (sel >> 2) & 1;

    let (out_1, out_2) = dmux_gate(inp, sel_3 != 0);

    let (out_3, out_4) = dmux_gate(out_1, sel_2 != 0);
    let (out_5, out_6) = dmux_gate(out_2, sel_2 != 0);

    let (a, b) = dmux_gate(out_3, sel_1 != 0);
    let (c, d) = dmux_gate(out_4, sel_1 != 0);
    let (e, f) = dmux_gate(out_5, sel_1 != 0);
    let (g, h) = dmux_gate(out_6, sel_1 != 0);

    let a_bit = match a {
        true => 1,
        false => 0,
    };

    let b_bit = match b {
        true => 1,
        false => 0,
    };

    let c_bit = match c {
        true => 1,
        false => 0,
    };

    let d_bit = match d {
        true => 1,
        false => 0,
    };

    let e_bit = match e {
        true => 1,
        false => 0,
    };

    let f_bit = match f {
        true => 1,
        false => 0,
    };

    let g_bit = match g {
        true => 1,
        false => 0,
    };

    let h_bit = match h {
        true => 1,
        false => 0,
    };

    (a_bit << 7)
        | (b_bit << 6)
        | (c_bit << 5)
        | (d_bit << 4)
        | (e_bit << 3)
        | (f_bit << 2)
        | (g_bit << 1)
        | (h_bit << 0)
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
        let res = dmux_gate(false, false);
        assert_eq!(res, (false, false));

        let res = dmux_gate(false, true);
        assert_eq!(res, (false, false));

        let res = dmux_gate(true, false);
        assert_eq!(res, (true, false));

        let res = dmux_gate(true, true);
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

    #[test]
    fn test_or8way() {
        let x: u8 = 0b00010000;
        let res = or8way_gate(x);
        assert_eq!(res, true);

        let x: u8 = 0b00000000;
        let res = or8way_gate(x);
        assert_eq!(res, false);

        let x: u8 = 0b00000011;
        let res = or8way_gate(x);
        assert_eq!(res, true);
    }

    #[test]
    fn test_mux4way16() {
        let a: u16 = 0b0001001000110100;
        let b: u16 = 0b1001100001110110;
        let c: u16 = 0b1010101010101010;
        let d: u16 = 0b0101010101010101;
        let sel: u8 = 0b00000000;
        let exp_out: u16 = 0b0001001000110100;

        let res = mux4way16_gate(a, b, c, d, sel);
        assert_eq!(res, exp_out);
    }

    #[test]
    fn test_mux8way16() {
        let a: u16 = 0b0001001000110100;
        let b: u16 = 0b0010001101000101;
        let c: u16 = 0b0011010001010110;
        let d: u16 = 0b0100010101100111;
        let e: u16 = 0b0101011001111000;
        let f: u16 = 0b0110011110001001;
        let g: u16 = 0b0111100010011010;
        let h: u16 = 0b1000100110101011;
        let sel: u8 = 0b000;
        let exp_out: u16 = 0b0001001000110100;

        let res = mux8way16_gate(a, b, c, d, e, f, g, h, sel);
        assert_eq!(res, exp_out);
    }

    #[test]
    fn test_dmux4way() {
        let inp = true;
        let sel: u8 = 0b00000000;
        let exp_out = (true, false, false, false);

        let res = dmux4way_gate(inp, sel);
        assert_eq!(res, exp_out);
    }

    #[test]
    fn test_dmux8way() {
        let inp: bool = false;
        let sel: u8 = 0b00000111;
        let res = dmux8way_gate(inp, sel);
        assert_eq!(res, 0b00000000);

        let inp: bool = true;
        let sel: u8 = 0b00000101;
        let res = dmux8way_gate(inp, sel);
        assert_eq!(res, 0b00000100);
    }
}
