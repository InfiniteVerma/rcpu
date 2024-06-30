
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
