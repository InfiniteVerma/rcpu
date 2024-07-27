pub fn u16_to_vec_bool(value: u16) -> Vec<bool> {
    let mut res: Vec<bool> = (0..16).map(|i| (value & (1 << i)) != 0).collect();
    res.reverse();
    res
}

pub fn vec_bool_to_u16(mut value: Vec<bool>) -> u16 {
    let mut res: u16 = 0;
    value.reverse();
    for i in 0..value.len() {
        let bit = match value[i] {
            false => 0,
            true => 1,
        };

        res |= bit << i;
    }

    res
}

pub fn vec_bool_to_u8(mut value: Vec<bool>) -> u8 {
    let mut res: u8 = 0;

    value.reverse();

    for i in 0..value.len() {
        let bit = match value[i] {
            false => 0,
            true => 1,
        };

        res |= bit << i;
    }

    res
}

pub fn u8_to_vec_bool(value: u8) -> Vec<bool> {
    let mut res: Vec<bool> = (0..8).map(|i| (value & (1 << i)) != 0).collect();
    res.reverse();
    res
}

pub fn u32_to_vec_bool(value: u32) -> Vec<bool> {
    let mut res: Vec<bool> = (0..32).map(|i| (value & (1 << i)) != 0).collect();
    res.reverse();
    res
}

pub fn vec_bool_to_u32(mut value: Vec<bool>) -> u32 {
    let mut res: u32 = 0;

    value.reverse();

    for i in 0..value.len() {
        let bit = match value[i] {
            false => 0,
            true => 1,
        };

        res |= bit << i;
    }

    res
}

#[cfg(test)]
mod tests {
    use crate::utils::{
        u16_to_vec_bool, u32_to_vec_bool, vec_bool_to_u16, vec_bool_to_u32, vec_bool_to_u8,
    };

    #[test]
    fn test_u16_to_vec_bool() {
        let inp: u16 = 0b1;
        let mut exp_out = vec![false; 16];
        exp_out[15] = true;

        assert_eq!(u16_to_vec_bool(inp), exp_out);

        let inp: u16 = 0b1111111111111111;
        let exp_out = vec![true; 16];

        assert_eq!(u16_to_vec_bool(inp), exp_out);
    }

    #[test]
    fn test_vec_bool_to_u16() {
        let inp = vec![true];
        let exp_out: u16 = 0b1;

        assert_eq!(vec_bool_to_u16(inp), exp_out);

        let inp = vec![true, false, true, false];
        let exp_out: u16 = 0b1010;

        assert_eq!(vec_bool_to_u16(inp), exp_out);
    }

    #[test]
    fn test_vec_bool_to_u8() {
        let inp = vec![true];
        let exp_out: u8 = 0b1;

        assert_eq!(vec_bool_to_u8(inp), exp_out);

        let inp = vec![true, false, false, false, true];
        let exp_out: u8 = 0b10001;

        assert_eq!(vec_bool_to_u8(inp), exp_out);

        let inp = vec![true, false, false, true, true];
        let exp_out: u8 = 0b10011;

        assert_eq!(vec_bool_to_u8(inp), exp_out);

        let inp = vec![true, false, true, true];
        let exp_out: u8 = 0b1011;

        assert_eq!(vec_bool_to_u8(inp), exp_out);
    }

    #[test]
    fn test_u32_to_vec_bool() {
        let inp: u32 = 0b1;
        let mut exp_out = vec![false; 32];
        exp_out[31] = true;

        assert_eq!(u32_to_vec_bool(inp), exp_out);

        let mut inp: u32 = 0b1111111111111111;
        inp = inp << 16 | 0b1111111111111111;
        let exp_out = vec![true; 32];

        assert_eq!(u32_to_vec_bool(inp), exp_out);
    }

    #[test]
    fn test_vec_bool_to_u32() {
        let inp = vec![true];
        let exp_out: u32 = 0b1;

        assert_eq!(vec_bool_to_u32(inp), exp_out);

        let inp = vec![true, false, false, false, true];
        let exp_out: u32 = 0b10001;

        assert_eq!(vec_bool_to_u32(inp), exp_out);
    }
}
