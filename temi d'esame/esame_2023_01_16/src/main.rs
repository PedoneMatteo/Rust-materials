/*
Una barriera è un costrutto di sincronizzazione usato per regolare l'avanzamento relativo della computazione di più thread.
All'atto della costruzione di questo oggetto, viene indicato il numero N di thread coinvolti.

Non è lecito creare una barriera che coinvolga meno di 2 thread.

La barriera offre un solo metodo, wait(), il cui scopo è bloccare temporaneamente l'esecuzione del thread che lo ha invocato, non ritornando fino a che non sono giunte
altre N-1 invocazioni dello stesso metodo da parte di altri thread: quando ciò succede, la barriera si sblocca e tutti tornano. Successive invocazioni del metodo wait()
hanno lo stesso comportamento: la barriera è ciclica.

Attenzione a non mescolare le fasi di ingresso e di uscita!

Una RankingBarrier è una versione particolare della barriera in cui il metodo wait() restituisce un intero che rappresenta l'ordine di arrivo: il primo thread ad avere
invocato wait() otterrà 1 come valore di ritorno, il secondo thread 2, e così via. All'inizio di un nuovo ciclo, il conteggio ripartirà da 1.

Si implementi la struttura dati RankingBarrier a scelta nei linguaggi Rust o C++ '11 o successivi.
*/

use std::collections::VecDeque;
use std::ops::Deref;
use std::sync::{Mutex, Condvar, Arc};

#[derive (Clone)]
struct RankingBarrier{
    num_threads: Arc<usize>,
    cv: Arc<Condvar>,
    counter: Arc<Mutex<usize>>,
}

impl RankingBarrier{
    fn new(n: usize)->Result<Self,()>{
        if n<2{
            Err(())
        }
        else{
           Ok( RankingBarrier{
               num_threads: Arc::new(n),
               cv : Arc::new(Condvar::new()),
               counter: Arc::new(Mutex::new(0)),
            })
        }

    }

    fn wait(&self, Thread_index: usize) -> usize{
        let mut lock = self.counter.lock().unwrap();
       // self.cv.wait_while(lock, |c| {c>0}).unwrap();
        (*lock)+=1;
        let ret = (*lock);
        println!("Thread {Thread_index} comes {}th", ret);
        lock = self.cv.wait_while(lock, |c| { c!=self.num_threads.deref()}).unwrap();
        self.cv.notify_all();
        (*lock)-=1;
        ret

    }
}

    fn main(){
        let c_barrier = RankingBarrier::new(5).expect("At least 2 threads are required for the barrier to work properly");
        let mut vt = Vec::new();

        for i in 0..5{
            vt.push(std::thread::spawn({
                let c = c_barrier.clone();
                move || {
                    for _ in 0..3 {
                        c.wait(i);
                    }
                }
            }
            ));
        }

        for t in vt {
            t.join().unwrap();
        }
    }

