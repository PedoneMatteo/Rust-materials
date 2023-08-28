/*àòò,
Un componente con funzionalità di cache permette di ottimizzare il comportamento di un sistema riducendo il numero di volte in cui una funzione è invocata,
tenendo traccia dei risultati da essa restituiti a fronte di un particolare dato in ingresso. Per generalità, si assuma che la funzione accetti un dato di
tipo generico K e restituisca un valore di tipo generico V.

Il componente offre un unico metodo get(...) che prende in ingresso due parametri, il valore k (di tipo K, clonabile) del parametro e la funzione f (di tipo K -> V)
responsabile della sua trasformazione, e restituisce uno smart pointer clonabile al relativo valore.

Se, per una determinata chiave k, non è ancora stato calcolato il valore corrispondente, la funzione viene invocata e ne viene restituito il risultato;
altrimenti viene restituito il risultato già trovato.

Il componente cache deve essere thread-safe perché due o più thread possono richiedere contemporaneamente il valore di una data chiave: quando questo avviene e il dato
non è ancora presente, la chiamata alla funzione dovrà essere eseguita nel contesto di UN SOLO thread, mentre gli altri dovranno aspettare il risultato in corso di
elaborazione, SENZA CONSUMARE cicli macchina.
*/
use std::fmt::Display;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;
use std::sync::{Mutex, Arc, Condvar, RwLock};
use rand::distributions::{Distribution, Standard};
use rand::Rng;

struct Cache<K: Eq + PartialEq + Hash + Clone,V>{
    map1: Mutex<HashMap<K,Arc<V>>>,
    map2: RwLock<HashMap<K,Arc<V>>>
}
impl<K: Eq + PartialEq + Hash + Clone,V> Cache<K,V>{
    fn new()-> Self{
        Cache{
            map1: Mutex::new(HashMap::new()),
            map2: RwLock::new(HashMap::new()),
        }
    }

    fn get(&self, k: K, func: impl Fn(K) -> V) -> Arc<V> {
      /*  let mut lock = self.map1.lock().unwrap();

        match (*lock).clone().get(&k) {
            Some(value) => {
                value.clone()
            }
            None => {
                let val = Arc::new(func(k.clone()));
                (*lock).insert(k.clone(), val.clone());
                return val;
            }
        }*/
        let mut rwlock = self.map2.read().unwrap();
        match (*rwlock).get(&k){
            Some(value) =>{
                return value.clone();
            }
            None => {
                drop(rwlock);
                let mut rwlock = self.map2.write().unwrap();
                if (*rwlock).get(&k).is_some(){
                    return (*rwlock).get(&k).unwrap().clone();
                }else{
                    let val = Arc::new(func(k.clone()));
                    (*rwlock).insert(k.clone(),val.clone());
                    return val;
                }
            }
        }

    }
}

pub fn f<K, V>(k:K)->V where Standard: Distribution<V>{
    let mut rng = rand::thread_rng();
    // Genera un numero intero casuale nell'intervallo [0, 100)
    let random_int: V = rng.gen();
    random_int
}


fn main() {
    println!("Hello, world!");
}
