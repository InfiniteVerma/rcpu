mod alu;
mod gates;
mod mem;
mod sequential;

//fn clock_tick(mut dff: DFF) {
    //let dff_data = [(false, false), (true, false), (false, true), (false, false)];
    //let mut i = 0;
    //loop {
    //    if i > 3 {
    //        i = 0;
    //    }

    //    let inp = dff_data[i].0;
    //    let exp_out = dff_data[i].1;
    //    thread::sleep(time::Duration::from_secs(1));
    //    println!("Tick");
    //    dff.clock(inp);
    //    let res = dff.get_state();
    //    println!("dff out: {}", res);
    //    assert_eq!(res, exp_out);

    //    i += 1;

    //    let inp = dff_data[i].0;
    //    let exp_out = dff_data[i].1;
    //    thread::sleep(time::Duration::from_secs(1));
    //    println!("Tock");
    //    dff.clock(inp);
    //    let res = dff.get_state();
    //    println!("dff out: {}", res);
    //    assert_eq!(res, exp_out);

    //    i += 1;
    //}
//}

//#[derive(Debug)]
//struct DFF {
//    state: bool,
//}
//
//impl DFF {
//
//    fn new() -> DFF {
//        DFF { state: false }
//    }
//
//    fn clock(&mut self, input: bool) {
//        self.state = input;
//    }
//
//    fn get_state(&self) -> bool {
//        self.state
//    }
//}

fn main() {
}

