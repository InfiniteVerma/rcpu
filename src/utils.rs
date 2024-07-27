pub fn u16_to_vec_bool(value: u16) -> Vec<bool> {
    (0..16).map(|i| (value & (1 << i)) != 0).collect()
}

pub fn vec_bool_to_u16(value: Vec<bool>) -> u16 {
    assert_eq!(value.len(), 16);

    let mut res: u16 = 0;
    for i in 0..value.len() {
        let bit = match value[i] {
            false => 0,
            true => 1,
        };

        res |= (bit << i);
    }

    res
}
