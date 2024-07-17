use crate::gates::{
    and16_gate, and_gate, mux16_gate, not16_gate, not_gate, or8way_gate, or_gate, xor_gate,
};

fn half_adder(a: bool, b: bool) -> (bool, bool) {
    //println!("half_adder => a: {}, b: {}", a, b);
    let sum = xor_gate(a, b);
    let carry = and_gate(a, b);

    (sum, carry)
}

fn full_adder(a: bool, b: bool, c: bool) -> (bool, bool) {
    //println!("full_adder => a: {}, b: {}, c: {}", a, b, c);
    let (sum_1, carry_1) = half_adder(a, b);
    let (sum, carry_2) = half_adder(sum_1, c);
    let carry = or_gate(carry_1, carry_2);

    (sum, carry)
}

fn add_16(a: u16, b: u16) -> u16 {
    //println!("Starting add_16 a: {}, b: {}", a, b);
    let a_bit = (a >> 0) & 1 != 0;
    let b_bit = (b >> 0) & 1 != 0;
    let (out_bool, mut c) = half_adder(a_bit, b_bit);
    let mut out: u16 = 0;

    match out_bool {
        true => {
            out |= 1;
        }
        false => {
            out |= 0;
        }
    };

    for i in 1..16 {
        let a_bit = (a >> i) & 1 != 0;
        let b_bit = (b >> i) & 1 != 0;

        let (out_bool, c_t) = full_adder(a_bit, b_bit, c);

        //println!("i: {} a: {} b: {} out_bool: {}", i, a_bit, b_bit, out_bool);
        c = c_t;

        match out_bool {
            true => {
                out |= 1 << i;
            }
            false => {
                out |= 0 << i;
            }
        };
    }

    out
}

fn inc_16(a: u16) -> u16 {
    add_16(a, 1)
}

/**
 * ALU (Arithmetic Logic Unit):
 * Computes out = one of the following functions:
 *                0, 1, -1,
 *                x, y, !x, !y, -x, -y,
 *                x + 1, y + 1, x - 1, y - 1,
 *                x + y, x - y, y - x,
 *                x & y, x | y
 * on the 16-bit inputs x, y,
 * according to the input bits zx, nx, zy, ny, f, no.
 * In addition, computes the two output bits:
 * if (out == 0) zr = 1, else zr = 0
 * if (out < 0)  ng = 1, else ng = 0
 */
// Implementation: Manipulates the x and y inputs
// and operates on the resulting values, as follows:
// if (zx == 1) sets x = 0        // 16-bit constant
// if (nx == 1) sets x = !x       // bitwise not
// if (zy == 1) sets y = 0        // 16-bit constant
// if (ny == 1) sets y = !y       // bitwise not
// if (f == 1)  sets out = x + y  // integer 2's complement addition
// if (f == 0)  sets out = x & y  // bitwise and
// if (no == 1) sets out = !out   // bitwise not
//
// CHIP ALU {
//    IN
//        x[16], y[16],  // 16-bit inputs
//        zx, // zero the x input?
//        nx, // negate the x input?
//        zy, // zero the y input?
//        ny, // negate the y input?
//        f,  // compute (out = x + y) or (out = x & y)?
//        no; // negate the out output?
//    OUT
//        out[16], // 16-bit output
//        zr,      // if (out == 0) equals 1, else 0
//        ng;      // if (out < 0)  equals 1, else 0
fn alu(
    x: u16,
    y: u16,
    zx: bool,
    nx: bool,
    zy: bool,
    ny: bool,
    f: bool,
    no: bool,
) -> (u16, bool, bool) {
    let zx_out = mux16_gate(x, 0, zx);
    let not_zx_out = not16_gate(zx_out);
    let xout_1 = mux16_gate(zx_out, not_zx_out, nx);

    let zy_out = mux16_gate(y, 0, zy);
    let not_zy_out = not16_gate(zy_out);
    let yout_1 = mux16_gate(zy_out, not_zy_out, ny);

    let xy_sum = add_16(xout_1, yout_1);
    let xy_and = and16_gate(xout_1, yout_1);
    let out_f = mux16_gate(xy_and, xy_sum, f);

    let not_out_f = not16_gate(out_f);
    let out = mux16_gate(out_f, not_out_f, no);
    let ng = (out >> 15) & 1 != 0;
    let tmp_out_1 = (out & 0b0000000011111111) as u8;
    let tmp_out_2 = ((out & 0b1111111100000000) >> 8) as u8;

    let tout_1 = or8way_gate(tmp_out_1);
    let tout_2 = or8way_gate(tmp_out_2);
    let tout = or_gate(tout_1, tout_2);
    let zr = not_gate(tout);

    (out, zr, ng)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_half_adder() {
        let test_data = [
            ((false, false), (false, false)),
            ((false, true), (true, false)),
            ((true, false), (true, false)),
            ((true, true), (false, true)),
        ];

        for data in test_data {
            let input_data = data.0;
            let exp_data = data.1;

            //println!("{:#?}", input_data);

            let res = half_adder(input_data.0, input_data.1);
            assert_eq!(res, exp_data);
        }
    }

    #[test]
    fn test_full_adder() {
        let test_data = [
            (false, false, false, false, false),
            (false, false, true, true, false),
            (false, true, false, true, false),
            (false, true, true, false, true),
            (true, false, false, true, false),
            (true, false, true, false, true),
            (true, true, false, false, true),
            (true, true, true, true, true),
        ];

        for data in test_data {
            let exp_data = (data.3, data.4);

            let res = full_adder(data.0, data.1, data.2);
            assert_eq!(res, exp_data);
        }
    }

    #[test]
    fn test_add_16() {
        let test_data = [
            (0b0000000000000000, 0b0000000000000000, 0b0000000000000000),
            (0b0000000000000000, 0b1111111111111111, 0b1111111111111111),
            (0b1111111111111111, 0b1111111111111111, 0b1111111111111110),
            (0b1010101010101010, 0b0101010101010101, 0b1111111111111111),
            (0b0011110011000011, 0b0000111111110000, 0b0100110010110011),
            (0b0001001000110100, 0b1001100001110110, 0b1010101010101010),
        ];

        for data in test_data {
            let exp_data = data.2;

            let res = add_16(data.0, data.1);
            println!("res: {:x} exp: {:x}", res, exp_data);
            assert_eq!(res, exp_data);
        }
    }

    #[test]
    fn test_inc_16() {
        let test_data = [
            (0b0000000000000000, 0b0000000000000001),
            (0b1111111111111111, 0b0000000000000000),
            (0b0000000000000101, 0b0000000000000110),
            (0b1111111111111011, 0b1111111111111100),
        ];

        for data in test_data {
            let exp = data.1;

            let res = inc_16(data.0);
            assert_eq!(res, exp);
        }
    }

    #[test]
    fn test_alu() {
        let test_data = [
            (
                0b0000000000000000,
                0b1111111111111111,
                1,
                0,
                1,
                0,
                1,
                0,
                0b0000000000000000,
            ),
            (
                0b0000000000000000,
                0b1111111111111111,
                1,
                1,
                1,
                1,
                1,
                1,
                0b0000000000000001,
            ),
            (
                0b0000000000000000,
                0b1111111111111111,
                1,
                1,
                1,
                0,
                1,
                0,
                0b1111111111111111,
            ),
            (
                0b0000000000000000,
                0b1111111111111111,
                0,
                0,
                1,
                1,
                0,
                0,
                0b0000000000000000,
            ),
            (
                0b0000000000000000,
                0b1111111111111111,
                1,
                1,
                0,
                0,
                0,
                0,
                0b1111111111111111,
            ),
            (
                0b0000000000000000,
                0b1111111111111111,
                0,
                0,
                1,
                1,
                0,
                1,
                0b1111111111111111,
            ),
            (
                0b0000000000000000,
                0b1111111111111111,
                1,
                1,
                0,
                0,
                0,
                1,
                0b0000000000000000,
            ),
            (
                0b0000000000000000,
                0b1111111111111111,
                0,
                0,
                1,
                1,
                1,
                1,
                0b0000000000000000,
            ),
            (
                0b0000000000000000,
                0b1111111111111111,
                1,
                1,
                0,
                0,
                1,
                1,
                0b0000000000000001,
            ),
            (
                0b0000000000000000,
                0b1111111111111111,
                0,
                1,
                1,
                1,
                1,
                1,
                0b0000000000000001,
            ),
            (
                0b0000000000000000,
                0b1111111111111111,
                1,
                1,
                0,
                1,
                1,
                1,
                0b0000000000000000,
            ),
            (
                0b0000000000000000,
                0b1111111111111111,
                0,
                0,
                1,
                1,
                1,
                0,
                0b1111111111111111,
            ),
            (
                0b0000000000000000,
                0b1111111111111111,
                1,
                1,
                0,
                0,
                1,
                0,
                0b1111111111111110,
            ),
            (
                0b0000000000000000,
                0b1111111111111111,
                0,
                0,
                0,
                0,
                1,
                0,
                0b1111111111111111,
            ),
            (
                0b0000000000000000,
                0b1111111111111111,
                0,
                1,
                0,
                0,
                1,
                1,
                0b0000000000000001,
            ),
            (
                0b0000000000000000,
                0b1111111111111111,
                0,
                0,
                0,
                1,
                1,
                1,
                0b1111111111111111,
            ),
            (
                0b0000000000000000,
                0b1111111111111111,
                0,
                0,
                0,
                0,
                0,
                0,
                0b0000000000000000,
            ),
            (
                0b0000000000000000,
                0b1111111111111111,
                0,
                1,
                0,
                1,
                0,
                1,
                0b1111111111111111,
            ),
            (
                0b0101101110100000,
                0b0001111011010010,
                1,
                0,
                1,
                0,
                1,
                0,
                0b0000000000000000,
            ),
            (
                0b0101101110100000,
                0b0001111011010010,
                1,
                1,
                1,
                1,
                1,
                1,
                0b0000000000000001,
            ),
            (
                0b0101101110100000,
                0b0001111011010010,
                1,
                1,
                1,
                0,
                1,
                0,
                0b1111111111111111,
            ),
            (
                0b0101101110100000,
                0b0001111011010010,
                0,
                0,
                1,
                1,
                0,
                0,
                0b0101101110100000,
            ),
            (
                0b0101101110100000,
                0b0001111011010010,
                1,
                1,
                0,
                0,
                0,
                0,
                0b0001111011010010,
            ),
            (
                0b0101101110100000,
                0b0001111011010010,
                0,
                0,
                1,
                1,
                0,
                1,
                0b1010010001011111,
            ),
            (
                0b0101101110100000,
                0b0001111011010010,
                1,
                1,
                0,
                0,
                0,
                1,
                0b1110000100101101,
            ),
            (
                0b0101101110100000,
                0b0001111011010010,
                0,
                0,
                1,
                1,
                1,
                1,
                0b1010010001100000,
            ),
            (
                0b0101101110100000,
                0b0001111011010010,
                1,
                1,
                0,
                0,
                1,
                1,
                0b1110000100101110,
            ),
            (
                0b0101101110100000,
                0b0001111011010010,
                0,
                1,
                1,
                1,
                1,
                1,
                0b0101101110100001,
            ),
            (
                0b0101101110100000,
                0b0001111011010010,
                1,
                1,
                0,
                1,
                1,
                1,
                0b0001111011010011,
            ),
            (
                0b0101101110100000,
                0b0001111011010010,
                0,
                0,
                1,
                1,
                1,
                0,
                0b0101101110011111,
            ),
            (
                0b0101101110100000,
                0b0001111011010010,
                1,
                1,
                0,
                0,
                1,
                0,
                0b0001111011010001,
            ),
            (
                0b0101101110100000,
                0b0001111011010010,
                0,
                0,
                0,
                0,
                1,
                0,
                0b0111101001110010,
            ),
            (
                0b0101101110100000,
                0b0001111011010010,
                0,
                1,
                0,
                0,
                1,
                1,
                0b0011110011001110,
            ),
            (
                0b0101101110100000,
                0b0001111011010010,
                0,
                0,
                0,
                1,
                1,
                1,
                0b1100001100110010,
            ),
            (
                0b0101101110100000,
                0b0001111011010010,
                0,
                0,
                0,
                0,
                0,
                0,
                0b0001101010000000,
            ),
            (
                0b0101101110100000,
                0b0001111011010010,
                0,
                1,
                0,
                1,
                0,
                1,
                0b0101111111110010,
            ),
        ];

        for data in test_data {
            let x = data.0;
            let y = data.1;
            let zx = data.2 != 0;
            let nx = data.3 != 0;
            let zy = data.4 != 0;
            let ny = data.5 != 0;
            let f = data.6 != 0;
            let no = data.7 != 0;
            let exp_data = data.8;
            let (res, _, _) = alu(x, y, zx, nx, zy, ny, f, no);

            assert_eq!(res, exp_data);
        }
    }
}
