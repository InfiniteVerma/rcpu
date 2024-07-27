use core::panic;
use std::thread;
use std::time::Duration;

use crate::gates::mux;
use crate::utils::{u16_to_vec_bool, vec_bool_to_u16};

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
    fn set_state(&mut self, inputs: Vec<bool>);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dff() {
        let mut clock = Clock::new();

        let dff1 = DFF::new();

        clock.register(Box::new(dff1));
        clock.set_state_of_component(0, vec![true]);
        let out = clock.get_state_of_component(0);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0], false);

        clock.tick();
        let out = clock.get_state_of_component(0);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0], true);

        clock.set_state_of_component(0, vec![false]);
        let out = clock.get_state_of_component(0);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0], true);

        clock.tick();
        let out = clock.get_state_of_component(0);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0], false);
    }

    #[test]
    fn test_bit() {
        let mut clock = Clock::new();

        let bit1 = Bit::new();

        clock.register(Box::new(bit1));
        let out = clock.get_state_of_component(0);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0], false);

        clock.set_state_of_component(0, vec![false, false]);

        clock.tick();
        let out = clock.get_state_of_component(0);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0], false);

        clock.set_state_of_component(0, vec![true, false]);

        clock.tick();
        let out = clock.get_state_of_component(0);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0], false);

        clock.set_state_of_component(0, vec![true, true]);

        clock.tick();
        let out = clock.get_state_of_component(0);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0], true);

        clock.set_state_of_component(0, vec![false, false]);

        clock.tick();
        let out = clock.get_state_of_component(0);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0], true);
    }

    #[test]
    fn test_register() {
        assert_eq!(1, 1);
        let mut clock = Clock::new();

        let reg1 = Register::new();

        clock.register(Box::new(reg1));
        let out = clock.get_state_of_component(0);
        assert_eq!(out.len(), 16);
        assert_eq!(out, [false; 16]);

        let inp_bool: u16 = 0b1000000010000000;
        let mut inp = u16_to_vec_bool(inp_bool);
        inp.extend([true]);
        clock.set_state_of_component(0, inp);
        clock.tick();

        let out = clock.get_state_of_component(0);
        assert_eq!(out.len(), 16);
        assert_eq!(vec_bool_to_u16(out), inp_bool);

        let inp_bool2: u16 = 0b1000000010000001;
        let mut inp2 = u16_to_vec_bool(inp_bool2);
        inp2.extend([false]);
        clock.set_state_of_component(0, inp2);
        clock.tick();

        let out = clock.get_state_of_component(0);
        assert_eq!(out.len(), 16);
        assert_eq!(vec_bool_to_u16(out), inp_bool);
    }
}
