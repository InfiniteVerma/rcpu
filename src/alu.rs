use crate::gates::{and_gate, or_gate, xor_gate};

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
}
