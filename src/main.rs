mod alu;
mod gates;
mod mem;
mod sequential;

use sequential::{Clock, DFF};

fn main() {
    let mut clock = Clock::new();

    let dff1 = DFF::new();

    clock.register(Box::new(dff1));
    clock.set_state_of_component(0, true);
    println!("State: {:#?}", clock.get_state_of_component(0));

    //clock.start_clock();
    clock.tick();
    println!("State: {:#?}", clock.get_state_of_component(0));
    clock.tick();
    println!("State: {:#?}", clock.get_state_of_component(0));
    clock.set_state_of_component(0, false);
    clock.tick();
    println!("State: {:#?}", clock.get_state_of_component(0));
    clock.tick();
}
