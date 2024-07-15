use crate::gates::{and_gate, or_gate, xor_gate};

fn half_adder(a: bool, b: bool) -> (bool, bool) {
    let sum = xor_gate(a, b);
    let carry = and_gate(a, b);

    (sum, carry)
}

fn full_adder(a: bool, b: bool, c: bool) -> (bool, bool) {
    let (sum_1, carry_1) = half_adder(a, b);
    let (sum, carry_2) = half_adder(sum_1, c);
    let carry = or_gate(carry_1, carry_2);

    (sum, carry)
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

            println!("{:#?}", input_data);

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
}
