/*
Un paradigma frequentemente usato nei sistemi reattivi e costituito dall'astrazione detta Looper.
Quando viene creato, un Looper crea una coda di oggetti generici di tipo Message ed un thread.
II thread attende - senza consumare cicli di CPU - che siano presenti messaggi nella coda,
li estrae a uno a uno nell'ordine di arrivo, e li elabora.

II costruttore di Looper riceve due parametri, entrambi di tipo (puntatore a) funzione: process( ... ) e cleanup().
La prima è una funzione responsabile di elaborare i singoli messaggi ricevuti attraverso la coda;
tale funzione accetta un unico parametro in ingresso di tipo Message e non ritorna nulla;
La seconda e funzione priva di argomenti e valore di ritorno e verra invocata dal thread incapsulato
nel Looper quando esso stara per terminare.

Looper offre un unico metodo pubblico, thread safe, oltre a quelli di servizio, necessari per gestirne ii ciclo di vita:
send(msg), che accetta come parametro un oggetto generico di tipo Message che verra inserito nella coda
e successivamente estratto dal thread ed inoltrato alla funzione di elaborazione.
Quando un oggetto Looper viene distrutto, occorre fare in modo che ii thread contenuto al suo interno
invochi la seconda funzione passata nel costruttore e poi termini.

Si implementi, utilizzando il linguaggio Rust o C++, tale astrazione tenendo conto che i suoi
 metodi dovranno essere thread-safe.
*/
use std::fmt::Debug;
use std::sync::{Mutex, Arc, Condvar};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use std::thread::{JoinHandle};

struct Looper<Msg>{
    queue: Mutex<Option<Sender<Msg>>>,
    thread: Option<JoinHandle<()>>,
}
impl<Msg: Send + Debug + 'static> Looper<Msg>{
    fn new(process: impl Fn(Msg)->() + Send + 'static, cleanup: impl Fn() + Send + 'static) -> Arc<Self>{
        let (tx,rx) = channel();
        let thread = thread::spawn({
            let process = process;
            let cleanup = cleanup;
            let receiver = rx;
            move || {
                while let Ok(msg) = receiver.recv(){
                    process(msg);
                }
                cleanup();
            }
        });
        Arc::new(
            Looper{
                queue: Mutex::new(Some(tx)),
                thread: Some(thread),
            }
        )
    }

    fn send(&self, msg: Msg){
        let mut lock = self.queue.lock().unwrap();
        let sender = (*lock).take().unwrap();
        sender.send(msg).unwrap();
        *lock = Some(sender);
    }
}

impl<Msg: Send + Debug> Drop for Looper<Message>{
    fn drop(&mut self){
        let sender = self.queue.lock().unwrap().take().unwrap();
        drop(sender);
        let join_handle = self.master_handle.take().unwrap();
        join_handle.join().unwrap();
    }
}


fn process<Message: Debug>(msg: Message) {
    println!("Processing {:?}", msg);
}

fn cleanup() {
    println!("cleaning...");
}

fn main() {
    println!("Hello, world!");
}
