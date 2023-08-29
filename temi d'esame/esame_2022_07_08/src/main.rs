/*
All'interno di un programma è necessario garantire che non vengano eseguite CONTEMPORANEAMENTE più di N invocazioni di operazioni potenzialmente lente.
A questo scopo, è stata definita la struttura dati ExecutionLimiter che viene inizializzata con il valore N del limite.
Tale struttura è thread-safe e offre solo il metodo pubblico generico execute( f ), che accetta come unico parametro una funzione f, priva di parametri
che ritorna il tipo generico R. Il metodo execute(...) ha, come tipo di ritorno, lo stesso tipo R restituito da f ed ha il compito di mantere il conteggio
di quante invocazioni sono in corso. Se tale numero è già pari al valore N definito all'atto della costruzione della struttura dati, attende, senza provocare
consumo di CPU, che scenda sotto soglia, dopodiché invoca la funzione f ricevuta come parametro e ne restituisce il valore. Poiché l'esecuzione della funzione f
potrebbe fallire, in tale caso, si preveda di decrementare il conteggio correttamente. Si implementi, usando i linguaggi Rust o C++, tale struttura dati,
garantendo tutte le funzionalità richieste.use std::sync::{Arc, Condvar, Mutex};
*/

use std::fmt::Error;
use std::ops::Deref;
use std::sync::{Arc, Condvar, Mutex};
use rand::Rng;
use rand::distributions::{Standard,Distribution};

struct ExecutionLimiter{
    limite: usize,
    cv: Condvar,
    contatore: Mutex<usize>,
}
impl ExecutionLimiter{

    fn new(N:usize)->Self{
        ExecutionLimiter{
            limite: N,
            cv: Condvar::new(),
            contatore: Mutex::new(0),
        }
    }

    fn execute<R: Clone>(&self, func: impl Fn()->R) ->Result<R,Error> {
        let mut lock = self.contatore.lock().unwrap();
        lock = self.cv.wait_while(lock, |x| {*x == self.limite}).unwrap();
        let ret :Result<R,Error> = Ok(func());
        match ret.is_ok(){
            true =>{
                *lock+=1;
                self.cv.notify_one();
                ret
            }
            false=>{
                *lock-=1;
                Err(Error)
            }
        }

    }
}
 pub fn f<R: Clone>()->Result<R,Error> where Standard: Distribution<R>{
     let mut r = rand::thread_rng();
     let val = r.gen();
     let v:Result<R,Error> = Ok(val.clone());
     if v.is_ok() {
         return Ok(val)
     }else { Err(Error) }
 }

fn main() {
    println!("Hello, world!");
}
