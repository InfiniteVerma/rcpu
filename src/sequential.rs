use core::panic;
use std::thread;
use std::time::Duration;

use crate::gates::{dmux8way_gate, mux, mux8way16_gate};
use crate::utils::{
    u16_to_vec_bool, u8_to_vec_bool, vec_bool_to_u16, vec_bool_to_u32, vec_bool_to_u8,
};

pub struct Clock {
    is_tick: bool,
    components: Vec<Box<dyn Tick>>,
}

impl Clock {
    pub fn new() -> Self {
        Clock {
            is_tick: false,
            components: Vec::new(),
        }
    }

    pub fn start_clock(&self) {
        let mut i = false;
        loop {
            if i {
                println!("Tick");
            } else {
                println!("Tock");
            }
            thread::sleep(Duration::from_secs(1));

            i = !i;
        }
    }

    // temporary func to manually tick
    pub fn tick(&mut self) {
        if self.is_tick {
            println!("Tick");
        } else {
            println!("Tock");
        }
        thread::sleep(Duration::from_secs(1));

        for component in &mut self.components {
            component.tick();
        }

        self.is_tick = !self.is_tick;
    }

    pub fn register(&mut self, component: Box<dyn Tick>) {
        self.components.push(component);
    }

    pub fn get_state_of_component(&self, index: usize) -> Vec<bool> {
        if let Some(val) = self
            .components
            .get(index)
            .map(|component| component.get_state())
        {
            val
        } else {
            panic!("Invalid index passed");
        }
    }

    pub fn set_state_of_component(&mut self, index: usize, inputs: Vec<bool>) {
        if let Some(component) = self.components.get_mut(index) {
            component.set_state(inputs);
        } else {
            panic!("Invalid index passed");
        }
    }
}

pub struct DFF {
    state: bool,
    next_state: bool, // updated on assignment but state gets updated on next tick only
}

pub trait Tick {
    fn tick(&mut self);
    fn get_state(&self) -> Vec<bool>;
    //fn set_state(&mut self, state: bool);
    fn set_state(&mut self, inputs: Vec<bool>); // TODO pass u64 and convert to Vec<bool> of size
                                                // required!
}

impl DFF {
    pub fn new() -> Self {
        DFF {
            state: false,
            next_state: false,
        }
    }

    pub fn update_state(&mut self) {
        self.state = self.next_state;
    }
}

impl Tick for DFF {
    fn tick(&mut self) {
        self.update_state();
    }

    fn get_state(&self) -> Vec<bool> {
        vec![self.state]
    }

    fn set_state(&mut self, inputs: Vec<bool>) {
        assert_eq!(inputs.len(), 1);
        self.next_state = inputs[0];
    }
}

/**
 * 1-bit register:
 * If load is asserted, the register's value is set to in;
 * Otherwise, the register maintains its current value:
 * if (load(t)) out(t+1) = in(t), else out(t+1) = out(t)
 */
struct Bit {
    dff: DFF,
    load: bool,
    state: bool,
}

impl Bit {
    pub fn new() -> Self {
        Bit {
            dff: DFF::new(),
            load: false,
            state: false,
        }
    }
}

impl Tick for Bit {
    fn tick(&mut self) {
        let dff_out = self.dff.get_state();

        assert_eq!(dff_out.len(), 1);

        let out_mux = mux(dff_out[0], self.state, self.load);
        self.dff.set_state(vec![out_mux]);
        self.dff.tick();
    }

    fn get_state(&self) -> Vec<bool> {
        let dff_out = self.dff.get_state();
        assert_eq!(dff_out.len(), 1);
        dff_out
    }

    fn set_state(&mut self, inputs: Vec<bool>) {
        assert_eq!(inputs.len(), 2);
        self.state = inputs[0];
        self.load = inputs[1];
    }
}

/**
 * 16-bit register:
 * If load is asserted, the register's value is set to in;
 * Otherwise, the register maintains its current value:
 * if (load(t)) out(t+1) = int(t), else out(t+1) = out(t)
 */
struct Register {
    bits: [Bit; 16],
}

impl Register {
    pub fn new() -> Self {
        let bits = [(); 16].map(|_| Bit::new());
        Register { bits }
    }
}

impl Tick for Register {
    fn tick(&mut self) {
        for i in 0..16 {
            self.bits[i].tick()
        }
    }

    fn get_state(&self) -> Vec<bool> {
        self.bits
            .iter()
            .map(|bit| {
                let bit_out = bit.get_state();
                assert_eq!(
                    bit_out.len(),
                    1,
                    "bit_out should have exactly one element but has {}",
                    bit_out.len()
                );
                bit_out[0]
            })
            .collect()
    }

    fn set_state(&mut self, inputs: Vec<bool>) {
        assert_eq!(inputs.len(), 16 + 1); // first 16-> 16bit input, next is the load bit

        let load = inputs[16];

        // 0 - 15
        for i in 0..16 {
            self.bits[i].set_state(vec![inputs[i], load]);
        }
    }
}

/**
 * Memory of eight 16-bit registers.
 * If load is asserted, the value of the register selected by
 * address is set to in; Otherwise, the value does not change.
 * The value of the selected register is emitted by out.
 */
struct RAM8 {
    registers: [Register; 8],
    addr: u8, // 3 bits
}

impl RAM8 {
    pub fn new() -> Self {
        let registers = [(); 8].map(|_| Register::new());
        RAM8 { registers, addr: 0 }
    }
}

impl Tick for RAM8 {
    fn tick(&mut self) {
        for i in 0..8 {
            self.registers[i].tick();
        }
    }

    fn get_state(&self) -> Vec<bool> {
        println!("get_state: self.addr: {}", self.addr);
        let res = mux8way16_gate(
            vec_bool_to_u16(self.registers[0].get_state()),
            vec_bool_to_u16(self.registers[1].get_state()),
            vec_bool_to_u16(self.registers[2].get_state()),
            vec_bool_to_u16(self.registers[3].get_state()),
            vec_bool_to_u16(self.registers[4].get_state()),
            vec_bool_to_u16(self.registers[5].get_state()),
            vec_bool_to_u16(self.registers[6].get_state()),
            vec_bool_to_u16(self.registers[7].get_state()),
            self.addr,
        );

        u16_to_vec_bool(res)
    }

    fn set_state(&mut self, inputs: Vec<bool>) {
        // 16 + 1 + 3
        assert_eq!(inputs.len(), 16 + 1 + 3);

        let load = inputs[16];
        let mut addr: Vec<bool> = vec![false; 3];

        for i in 0..3 {
            addr[i] = inputs[i + 17];
        }

        self.addr = vec_bool_to_u8(addr.clone());

        println!(
            "ANANT load: {}, addr: {:#?}, self.addr: {:#?}",
            load, addr, self.addr
        );

        let dmux_out: Vec<bool> = u8_to_vec_bool(dmux8way_gate(load, vec_bool_to_u8(addr)));

        /*
         * Input is vector of bool. Here converting to u32
         *  - Then right shift 4 and take the 16 LSB bits as input data
         */
        let input_bit: u32 = vec_bool_to_u32(inputs.clone()) >> 4;
        let inp_bool = u16_to_vec_bool((input_bit & 0xFFFF) as u16);

        assert_eq!(inp_bool.len(), 16);

        for i in 0..8 {
            let mut input = inp_bool.clone();
            input.extend([dmux_out[i]]);
            self.registers[i].set_state(input);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::u32_to_vec_bool;

    use super::*;

    //#[test]
    //fn test_dff() {
    //    let mut clock = Clock::new();

    //    let dff1 = DFF::new();

    //    clock.register(Box::new(dff1));
    //    clock.set_state_of_component(0, vec![true]);
    //    let out = clock.get_state_of_component(0);
    //    assert_eq!(out.len(), 1);
    //    assert_eq!(out[0], false);

    //    clock.tick();
    //    let out = clock.get_state_of_component(0);
    //    assert_eq!(out.len(), 1);
    //    assert_eq!(out[0], true);

    //    clock.set_state_of_component(0, vec![false]);
    //    let out = clock.get_state_of_component(0);
    //    assert_eq!(out.len(), 1);
    //    assert_eq!(out[0], true);

    //    clock.tick();
    //    let out = clock.get_state_of_component(0);
    //    assert_eq!(out.len(), 1);
    //    assert_eq!(out[0], false);
    //}

    //#[test]
    //fn test_bit() {
    //    let mut clock = Clock::new();

    //    let bit1 = Bit::new();

    //    clock.register(Box::new(bit1));
    //    let out = clock.get_state_of_component(0);
    //    assert_eq!(out.len(), 1);
    //    assert_eq!(out[0], false);

    //    clock.set_state_of_component(0, vec![false, false]);

    //    clock.tick();
    //    let out = clock.get_state_of_component(0);
    //    assert_eq!(out.len(), 1);
    //    assert_eq!(out[0], false);

    //    clock.set_state_of_component(0, vec![true, false]);

    //    clock.tick();
    //    let out = clock.get_state_of_component(0);
    //    assert_eq!(out.len(), 1);
    //    assert_eq!(out[0], false);

    //    clock.set_state_of_component(0, vec![true, true]);

    //    clock.tick();
    //    let out = clock.get_state_of_component(0);
    //    assert_eq!(out.len(), 1);
    //    assert_eq!(out[0], true);

    //    clock.set_state_of_component(0, vec![false, false]);

    //    clock.tick();
    //    let out = clock.get_state_of_component(0);
    //    assert_eq!(out.len(), 1);
    //    assert_eq!(out[0], true);
    //}

    //#[test]
    //fn test_register() {
    //    let mut clock = Clock::new();

    //    let reg1 = Register::new();

    //    clock.register(Box::new(reg1));
    //    let out = clock.get_state_of_component(0);
    //    assert_eq!(out.len(), 16);
    //    assert_eq!(out, [false; 16]);

    //    let inp_bool: u16 = 0b1000000010000000;
    //    let mut inp = u16_to_vec_bool(inp_bool);
    //    inp.extend([true]);
    //    clock.set_state_of_component(0, inp);
    //    clock.tick();

    //    let out = clock.get_state_of_component(0);
    //    assert_eq!(out.len(), 16);
    //    assert_eq!(vec_bool_to_u16(out), inp_bool);

    //    let inp_bool2: u16 = 0b1000000010000001;
    //    let mut inp2 = u16_to_vec_bool(inp_bool2);
    //    inp2.extend([false]);
    //    clock.set_state_of_component(0, inp2);
    //    clock.tick();

    //    let out = clock.get_state_of_component(0);
    //    assert_eq!(out.len(), 16);
    //    assert_eq!(vec_bool_to_u16(out), inp_bool);
    //}

    #[test]
    fn test_ram8() {
        let mut clock = Clock::new();

        let ram8 = RAM8::new();

        clock.register(Box::new(ram8));
        let out = clock.get_state_of_component(0);
        assert_eq!(out.len(), 16);
        assert_eq!(out, [false; 16]);

        /*
         * Test 1: Set and get register at addr 010
         */
        let input: u32 = 0b1000000011111111;
        let load = 1;
        let addr = 0b101;
        let mut final_input: u32 = input;
        final_input = (final_input << 1) | load;
        final_input = (final_input << 3) | addr;

        let inp = u32_to_vec_bool(final_input)[12..].to_vec();
        clock.set_state_of_component(0, inp);
        clock.tick();

        let out = clock.get_state_of_component(0);
        assert_eq!(out.len(), 16);
        assert_eq!(vec_bool_to_u16(out), (input & 0xFFFF) as u16);

        /*
         * Test 2: Read it after another tick, load ==0 at same addr
         */
        let input: u32 = 0;
        let load = 0;
        let addr = 0b101;
        let final_input: u32 = (((input << 1) | load) << 3) | addr;

        let inp = u32_to_vec_bool(final_input)[12..].to_vec();
        clock.set_state_of_component(0, inp);
        clock.tick();

        let out = clock.get_state_of_component(0);
        assert_eq!(out.len(), 16);
        assert_eq!(vec_bool_to_u16(out), 0b1000000011111111);

        /*
         * Test 3: Read a different addr
         */
        let input: u32 = 0;
        let load = 0;
        let addr = 0b100;
        let final_input: u32 = (((input << 1) | load) << 3) | addr;

        let inp = u32_to_vec_bool(final_input)[12..].to_vec();
        clock.set_state_of_component(0, inp);
        clock.tick();

        let out = clock.get_state_of_component(0);
        assert_eq!(out.len(), 16);
        assert_eq!(vec_bool_to_u16(out), 0);
    }
}
