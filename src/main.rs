mod gates;

use gates::{and_gate, nand_gate, not_gate, or_gate};

fn main() {
    println!("AND GATE: {}", and_gate(true, false));
    println!("OR GATE: {}", or_gate(true, false));
    println!("NOT GATE: {}", not_gate(true));
    println!("NAND GATE: {}", nand_gate(true, false));
}
