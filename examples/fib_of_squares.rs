
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

    let squarecr = Coroutine::new(move |cr| {
            loop {
                match fibcr.next(){
                    Some(fib) => cr.produce(fib*fib),
                    None      => {return ();},
                }
            }
        });

    for sqr in squarecr.iter(){
        println!("{}", sqr);
    }
}
