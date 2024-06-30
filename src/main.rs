fn main() {
    let bit_true: bool = true;

    let bits: u8 = 0b00000101;

    let bit0 = bits & 0b00000001;
    let bit1 = bits & 0b00000010;

    println!("{}", bit0);
    println!("{}", bit1);
}
