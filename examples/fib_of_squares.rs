
extern crate coroutine_simple;

use coroutine_simple::Coroutine;

use std::sync::Arc;

fn main() {

    const MANY: u64 = 15;

    let fibcr = Arc::new(Coroutine::new(|cr| {
                let mut a = 1;
                let mut b = 1;
                for _ in 0..MANY{
                    cr.produce(b);
                    let c = a;
                    a = b;
                    b += c;
                }
            }));

    let squarerc = Coroutine::new(move |cr| {
            loop {
                let fib = fibcr.next();
                cr.produce(fib*fib);
            }
        });

    loop {
        println!("{}", squarerc.next());
    }
}
