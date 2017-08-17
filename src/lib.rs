
use std::collections::VecDeque;
use std::thread::{self, Thread};
use std::sync::{Mutex, Arc};

pub struct Coroutine <T: Send + 'static>{
    buffer: Arc<Mutex<VecDeque<T>>>,
    waiting_threads: Arc<Mutex<VecDeque<Thread>>>,
}

impl <T> Coroutine<T>
            where   T: Send + 'static{

    pub fn new<F: Fn(Arc<Self>) + Send + 'static>(p_func: F) -> Arc<Self> {
        let cr = Arc::new(Coroutine {
            buffer: Arc::new(Mutex::new(VecDeque::new())),
            waiting_threads: Arc::new(Mutex::new(VecDeque::new())),
        });

        let l_cr = cr.clone();
        thread::spawn(move || p_func(l_cr));
        cr
    }

    pub fn next(&self) -> T {
        loop {
            match self.buffer.lock().unwrap().pop_front(){
                Some(t) => return t,
                None    => (),
            }
            self.waiting_threads.lock().unwrap().push_back(thread::current());
            thread::park();
        }
    }

    pub fn produce(&self, data: T) {
        self.buffer.lock().unwrap().push_back(data);
        match self.waiting_threads.lock().unwrap().pop_front(){
                Some(thr) => thr.unpark(),
                None      => (),
        }
    }
}

unsafe impl <T> Sync for Coroutine<T>
            where   T: Send + 'static{}

