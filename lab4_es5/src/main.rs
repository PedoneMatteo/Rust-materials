use std::sync::{Arc, Barrier};
use std::ops::{Deref, DerefMut};
use std::thread;

pub struct CyclicBarrier {
    barrier: Arc<Barrier>,
    num_threads: usize,
    num_wait: usize,
}

impl CyclicBarrier{
    pub fn new(num_threads: usize) -> CyclicBarrier {
        CyclicBarrier {
            barrier : Arc::new(Barrier::new(num_threads)),
            num_threads,
            num_wait: 0,
        }
    }
    pub fn wait(&mut self){
        self.num_wait += 1;
        if self.num_wait == self.num_threads {
            self.barrier.wait();
            self.num_wait = 0;
        }else{
            self.barrier.wait();
        }
    }

    pub fn get_number_of_threads(&self) -> usize {
        self.num_threads
    }

    pub fn get_number_waiting(&self) -> usize {
        self.num_wait
    }
}
/*
impl Deref for CyclicBarrier {
    type Target = Barrier;

    fn deref(&self) -> &Self::Target {
        &self.barrier
    }
}

impl DerefMut for CyclicBarrier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        Arc::get_mut(&mut self.barrier).expect("Failed to get mutable reference to Barrier")
    }
}
*/

fn main() {
    let abarrier = Arc::new(CyclicBarrier::new(3));
    let mut vt = Vec::new();
    for i in 0..3 {
        //let mut cbarrier:Arc<CyclicBarrier> = Arc::clone(&abarrier);
       let mut cbarrier = abarrrier.clone();
        vt.push(std::thread::spawn(move || {
            for j in 0..10 {
                cbarrier.wait();
                println!("after barrier {} {}\n", i, j);
            }
        }));
    }
    for t in vt {
        t.join().unwrap();
    }
}