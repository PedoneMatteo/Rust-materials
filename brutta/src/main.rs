enum List {
    Cons(i32, Box<List>),
    Nil,
}
use crate::List::{Cons, Nil};
use std::rc::Rc;
use std::sync::{Arc, Condvar, Mutex};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

fn main() {
   /* let a = Cons(5, Box::new(Cons(10,Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(b));
    let five = Rc::new(5);
    let weak_five = Rc::downgrade(&five);
    let strong_five = weak_five.upgrade();
    let c = strong_five.unwrap();
*/
/*
    let v = std::sync::Mutex::new(vec![1,2,3]);
    std::thread::scope(|s|{
        s.spawn(||{
            for i in 10..20{
                let mut lg = v.lock().unwrap();
                lg.push(i);
            }
        });
        s.spawn(||{
            for i in 30..40{
                v.lock().unwrap().push(i);
            }
        });

    });

    let v = v.lock().unwrap();
    for i in v.to_vec(){
        println!("{}",i);
    }
*/
    /*
    let mut x =0;
    let pair = Arc::new((Mutex::new(x), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    for i in 0..=10 {
        println!("Ciclo: {}", i);
        let pair2_clone = Arc::clone(&pair2);
        let pair_clone = Arc::clone(&pair);
        std::thread::spawn(move || {
            let (lock, cvar) = &*pair2_clone;
            let mut started = lock.lock().unwrap();
            if i>4{
                *started = *started+2;
            }else{
                *started = *started+1;
            }

            cvar.notify_one();
            println!("{}: Notify one! -> {}", i, *started);
        });
        std::thread::spawn(move || {
            let (lock, cvar) = &*pair_clone;
            let mut started = lock.lock().unwrap();
            while *started<5 {
                started = cvar.wait(started).unwrap();
            }
            *started= *started-1;
            println!("{}: After wait! -> {}", i, *started);
        });
    }

     */
/*
    let pair = Arc::new((Mutex::new(Vec::<i32>::new()),Condvar::new()));
    let pair2 = pair.clone();
    let t = thread::spawn(move ||{
        let (m,cv) = &*pair2;
        for i in 0..10{
            thread::sleep(Duration::from_secs(3));
            let mut v = m.lock().unwrap();
            v.push(i);
            cv.notify_all();
        }
    });
    let (m,cv) = &*pair;
    let mut round = 0;
    while round != 10{
        let mut v = m.lock().unwrap();
        while round == v.len(){
            v = cv.wait(v).unwrap();
        }
        println!("While sleeping {} values have been produced", v.len() - round );
        for i in round..v.len(){
            println!("{}", v[i]);
        }
        round = v.len();
    }
    t.join().unwrap();
 */
    let (tx,rx) = channel();
    let t1 = thread::spawn(move || {
        for i in 0..10{
            thread::sleep(Duration::from_secs(2));
            tx.send(i).unwrap();
        }
    });
    while let Ok(i) = rx.recv(){
        println!("Ricevuto {i}");
    }
    t1.join().unwrap();

}

