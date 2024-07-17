mod alu;
mod gates;
mod mem;
use std::{thread, time};

fn clock_tick(mut dff: Dff) {
    let dff_data = [(false, false), (true, false), (false, true), (false, false)];
    let mut i = 0;
    loop {
        if i > 3 {
            i = 0;
        }

        let inp = dff_data[i].0;
        let exp_out = dff_data[i].1;
        thread::sleep(time::Duration::from_secs(1));
        println!("Tick");
        let res = dff.exec(inp);
        println!("dff out: {}", res);
        assert_eq!(res, exp_out);

        i += 1;

        let inp = dff_data[i].0;
        let exp_out = dff_data[i].1;
        thread::sleep(time::Duration::from_secs(1));
        println!("Tock");
        let res = dff.exec(inp);
        println!("dff out: {}", res);
        assert_eq!(res, exp_out);

        i += 1;
    }
}

#[derive(Debug)]
struct Dff {
    state: bool,
}

impl Dff {
    fn exec(&mut self, input: bool) -> bool {
        let old_state = self.state;
        self.state = input;
        old_state
    }
}

fn main() {
    let dff = Dff { state: false };
    clock_tick(dff);
}
