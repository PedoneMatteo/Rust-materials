/*
In un sistema concorrente, ciascun thread può pubblicare eventi per rendere noto ad altri thread quanto sta facendo.
Per evitare un accoppiamento stretto tra mittenti e destinatari degli eventi, si utilizza un Dispatcher: questo è un oggetto thread-safe che offre il metodo

        dispatch(msg: Msg)

mediante il quale un messaggio di tipo generico Msg (soggetto al vincolo di essere clonabile) viene reso disponibile a chiunque si sia sottoscritto.
Un thread interessato a ricevere messaggi può invocare il metodo

        subscribe()

del Dispatcher: otterrà come risultato un oggetto di tipo Subscription mediante il quale potrà leggere i messaggi che da ora in poi saranno pubblicati attraverso
il Dispatcher. Per ogni sottoscrizione attiva, il Dispatcher mantiene internamente l'equivalente di una coda ordinata (FIFO) di messaggi non ancora letti.
A fronte dell'invocazione del metodo dispatch(msg:Msg), il messaggio viene clonato ed inserito in ciascuna delle code esistenti. L'oggetto Subscription offre il
metodo bloccante

        read() -> Option<Msg>

se nella coda corrispondente è presente almeno un messaggio, questo viene rimosso e restituito; se nella coda non è presente nessun messaggio e il Dispatcher esiste
ancora, l'invocazione si blocca fino a che non viene inserito un nuovo messaggio; se invece il Dispatcher è stato distrutto, viene restituito il valore corrispondente
all'opzione vuota.

Gli oggetti Dispatcher e Subscription sono in qualche modo collegati, ma devono poter avere cicli di vita indipendenti: la distruzione del Dispatcher non deve impedire la
consumazione dei messaggi già recapitati ad una Subscription, ma non ancora letti; parimenti, la distruzione di una Subscription non deve impedire al Dispatcher di
consegnare ulteriori messaggi alle eventuali altre Subscription presenti.

Si implementino le strutture dati Dispatcher e Subscription, a scelta, nel linguaggio Rust o C++11.
*/
use std::collections::VecDeque;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};

struct Dispatcher<Msg: Clone>{
    queue: Mutex<VecDeque<Sender<Msg>>>
}

struct Subscription<Msg: Clone>{
    rec: Receiver<Msg>,
}

impl<Msg: Clone> Dispatcher<Msg>{
    fn new()-> Self{
        Dispatcher{
            queue: Mutex::new(VecDeque::new()),
        }
    }

    fn dispatch(&self, msg:Msg){
        let mut lock = self.queue.lock().unwrap();
        for sender in lock.iter(){
            let _ = sender.send(msg.clone());
        }
    }

    fn subscribe(&self) -> Subscription<Msg> {
        let (tx,rx) = channel();
        let mut lock = self.queue.lock().unwrap();
        (*lock).push_back(tx);
        Subscription::new(rx)
    }
}


impl<Msg: Clone> Subscription<Msg>{
    fn new(rx: Receiver<Msg>) -> Self{
        Subscription{
            rec: rx,
        }
    }

    fn read(&self) -> Option<Msg>{
        let msg = self.rec.recv();
        if msg.is_ok() {
            Some(msg.unwrap())
        } else {
            None
        }
    }
}

fn main() {
    println!("Hello, world!");
}
