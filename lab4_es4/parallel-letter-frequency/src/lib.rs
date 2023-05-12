use std::collections::HashMap;
use std::cmp::min;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    /*
    let characters = "abcdefghijklmnopqrstuvwxyz";
    let alphabet = characters.chars().collect::<Vec<char>>();
    */
    let mut frequencies :HashMap<char,usize> = HashMap::new();

    let input = input.join(""); //fa diventare string tutto il vettore
    if input.len() == 0 {
        return frequencies;
    }

    let real_worker_count = min(input.len(), worker_count); //(?)
    let mut thread_pool = Vec::with_capacity(real_worker_count);

    for i in 0..real_worker_count {
        let character = input.chars().nth(i).unwrap();  //nth() serve per accedere a un certo indice della stringa
        let input = input.clone();

        let my_thread = thread::spawn( move || {
            let mut check:(char, usize) = (character, 0);
            for ch in input.chars(){
                if ch == character{
                    check.1 += 1;
                }
            }
            check
        });
        thread_pool.push(my_thread);
    }

    for my_thread in thread_pool {
        let tupla = my_thread.join().unwrap();  //A,1
        let opt = frequencies.get(&(tupla.0.to_ascii_lowercase())); //a,1

        match opt{
            Some(opt) => {  //valore già presente nella hashmap
                let current_key = *frequencies.get_key_value(&tupla.0.to_ascii_lowercase()).unwrap().0; //a

                if tupla.0 == current_key.to_ascii_uppercase() { //A ?= A
                    frequencies.insert(current_key, tupla.1 + opt); //a, 1 + 1
                }
            },
            None => {
                if tupla.1 != 0 {
                    frequencies.insert(tupla.0.to_ascii_lowercase(), tupla.1);
                }
            }
        }
    }

    frequencies
}
/*
use std::collections::HashMap;
use std::cmp::min;
use std::thread;

const ODE_AN_DIE_FREUDE: [&str; 8] = [
    "Freude schöner Götterfunken",
    "Tochter aus Elysium,",
    "Wir betreten feuertrunken,",
    "Himmlische, dein Heiligtum!",
    "Deine Zauber binden wieder",
    "Was die Mode streng geteilt;",
    "Alle Menschen werden Brüder,",
    "Wo dein sanfter Flügel weilt.",
];
// Dutch national anthem
const WILHELMUS: [&str; 8] = [
    "Wilhelmus van Nassouwe",
    "ben ik, van Duitsen bloed,",
    "den vaderland getrouwe",
    "blijf ik tot in den dood.",
    "Een Prinse van Oranje",
    "ben ik, vrij, onverveerd,",
    "den Koning van Hispanje",
    "heb ik altijd geëerd.",
];
// American national anthem
const STAR_SPANGLED_BANNER: [&str; 8] = [
    "O say can you see by the dawn's early light,",
    "What so proudly we hailed at the twilight's last gleaming,",
    "Whose broad stripes and bright stars through the perilous fight,",
    "O'er the ramparts we watched, were so gallantly streaming?",
    "And the rockets' red glare, the bombs bursting in air,",
    "Gave proof through the night that our flag was still there;",
    "O say does that star-spangled banner yet wave,",
    "O'er the land of the free and the home of the brave?",
];

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    /*
    let characters = "abcdefghijklmnopqrstuvwxyz";
    let alphabet = characters.chars().collect::<Vec<char>>();
    */
    let mut frequencies :HashMap<char,usize> = HashMap::new();

    let input = input.join(""); //fa diventare string tutto il vettore
    if input.len() == 0 {
        return frequencies;
    }

    let real_worker_count = min(input.len(), worker_count); //(?)
    let mut thread_pool = Vec::with_capacity(real_worker_count);

    for i in 0..real_worker_count {
        let chunk = input.chars().by_ref().take(input.len() / real_worker_count + 1).collect::<String>(); //con questo take divido il vettore in tanti pezzi quanti sono i worker
        let character = input.chars().nth(i).unwrap();  //nth() serve per accedere a un certo indice della stringa
        let input = input.clone();

        let my_thread = thread::spawn( move || {
            let mut answer = HashMap::<char, usize>::new();
            chunk.chars().for_each(|c| {
                if c.is_alphabetic() {
                    *answer.entry(c.to_ascii_lowercase()).or_default() += 1;
                }
            });
            answer
            /*let mut check = (character, 0);
            for ch in input.chars(){
                if ch == character{
                    check.1 += 1;
                }
            }
            check*/
        });
        thread_pool.push(my_thread);
    }

    for my_thread in thread_pool {
        let answer = my_thread.join().unwrap();
        for (key, val) in answer.iter() {
            *frequencies.entry(*key).or_default() += val;
        }
    }

        /*
        let tupla = my_thread.join().unwrap();  //A,1
        let opt = frequencies.get(&(tupla.0.to_ascii_lowercase())); //a,1

        match opt{
            Some(opt) => {  //valore già presente nella hashmap
                let current_key = *frequencies.get_key_value(&tupla.0.to_ascii_lowercase()).unwrap().0; //a

                if tupla.0 == current_key.to_ascii_uppercase() {
                    frequencies.insert(current_key, tupla.1 + opt);
                }
            },
            None => {
                if tupla.1 != 0 {
                    frequencies.insert(tupla.0.to_ascii_lowercase(), tupla.1);
                }
            }
        }
   } */

    frequencies

}

fn main(){
     let mut v = Vec::new();
     for anthem in [ODE_AN_DIE_FREUDE, WILHELMUS, STAR_SPANGLED_BANNER].iter() {
         for line in anthem.iter() {
             v.push(*line);
         }
     }
     let freqs = frequency(&v[..], 3);
     println!("{:?}", freqs.get(&'a'));
     /*assert_eq!(freqs.get(&'a'), Some(&49));
     assert_eq!(freqs.get(&'t'), Some(&56));
     assert_eq!(freqs.get(&'ü'), Some(&2));*/






 }
*/