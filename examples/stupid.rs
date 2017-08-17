extern crate coroutine_simple;

use coroutine_simple::Coroutine;

use std::thread;

fn main() {
    let cr = Coroutine::new(|cr| {
            for i in 0..10000 {
                cr.produce(i*i);
            }
            println!("cr, produced all data");
        });

    thread::sleep_ms(1000);
    loop {
        println!(" {} ", cr.next());
    }
}
