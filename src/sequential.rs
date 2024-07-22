use core::panic;
use std::thread;
use std::time::Duration;

trait Sequentail {}

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

    pub fn get_state_of_component(&self, index: usize) -> bool {
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

    pub fn set_state_of_component(&mut self, index: usize, state: bool) {
        if let Some(component) = self.components.get_mut(index) {
            component.set_state(state);
        } else {
            panic!("Invalid index passed");
        }
    }
}

pub trait Tick {
    fn tick(&mut self);
    fn get_state(&self) -> bool;
    fn set_state(&mut self, state: bool);
}

impl Tick for DFF {
    fn tick(&mut self) {
        self.update_state();
    }

    fn get_state(&self) -> bool {
        self.state
    }

    fn set_state(&mut self, state: bool) {
        self.next_state = state;
    }
}

pub struct DFF {
    state: bool,
    next_state: bool, // updated on assignment but state gets updated on next tick only
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
