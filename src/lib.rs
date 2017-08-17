
use std::collections::VecDeque;
use std::thread::{self, Thread};
use std::sync::{Mutex, Arc};
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Clone)]
pub struct Coroutine <T: Send + 'static + Clone>{
    buffer: Arc<Mutex<VecDeque<T>>>,
    waiting_threads: Arc<Mutex<VecDeque<Thread>>>,
    running: Arc<AtomicBool>,
}

impl <T> Coroutine<T>
            where   T: Send + 'static + Clone{

    pub fn new<F: Fn(Arc<Self>) + Send + 'static>(p_func: F) -> Arc<Self> {
        let cr = Arc::new(Coroutine {
            buffer: Arc::new(Mutex::new(VecDeque::new())),
            waiting_threads: Arc::new(Mutex::new(VecDeque::new())),
            running: Arc::new(AtomicBool::new(true)),
        });

        let l_cr = cr.clone();
        thread::spawn(move || {
                p_func(l_cr.clone());
                l_cr.end();
            });
        cr
    }

    pub fn end(&self){
        self.running.store(false, Ordering::Relaxed);
        for thr in self.waiting_threads.lock().unwrap().iter(){
            thr.unpark();
        }
    }

    pub fn next(&self) -> Option<T> {
        loop {
           
            match self.buffer.lock().unwrap().pop_front(){
                Some(t) => return Some(t),
                None    => {
                        if !self.running.load(Ordering::Relaxed) {
                            return None;
                        }
                        ()
                    },
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

    pub fn iter(&self) -> CrIter<T> {
        CrIter(self.to_owned())
    }
}

pub struct CrIter<T: Send + 'static + Clone> (Coroutine<T>);
impl<T> Iterator for CrIter<T> 
        where T: Send+ 'static + Clone{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.0.next()
    }
}

unsafe impl <T> Sync for Coroutine<T>
            where   T: Send + 'static + Clone{}

