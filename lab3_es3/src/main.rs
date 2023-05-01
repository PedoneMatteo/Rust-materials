use std::env::args;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};
//use chrono::NaiveTime;

#[derive(Debug)]
struct Calendar{
    schedule: Vec<(u32, u32)>,
    bounds: (u32,u32)       //tupla
}

impl Calendar {

    fn from_File(f: File) -> Self {
        let mut bounds: (u32, u32) = (0, 0);
        let mut interval: (u32, u32) = (0, 0);
        let mut schedule: Vec<(u32, u32)> = Vec::new();

        let buffered = BufReader::new(f);

        for (i, line) in buffered.lines().enumerate() {
            match line {
                Err(e) => panic! {"errore lettura riga file"},
                Ok(s) => {
                    let time = s.trim().split(":")
                        .map(|x| x.parse::<u32>().unwrap())
                        .reduce(|accumulator, element| accumulator * 60 + element)
                        //nella prima iterazione l'accumulator è 0 e element è 09 e calcola 0*60+09, poi l'accumulator sarà 0*60+09 e element è 00 e cosi via..
                        .unwrap();
                    if i == 0 {
                        bounds.0 = time;
                    } else if i == 1 {
                        bounds.1 = time;
                    } else if i % 2 == 0 {
                        interval.0 = time;
                    } else {
                        interval.1 = time;
                        schedule.push(interval);
                    }
                }
            }
        }
        Self{schedule, bounds}
    }

    fn true_buchi (&self, durata: u32, times: Vec<(u32, u32)>) -> Vec<(u32, u32)>{
        let mut schedule: Vec<(u32, u32)> = Vec::new();

        for i in (1..times.len()){
            if self.schedule[i].0 - self.schedule[i-1].1 > durata {

            }
        }


        schedule
    }

}

fn buchi(durata: u32, times: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    let mut schedule: Vec<(u32, u32)> = Vec::new();

    for i in (1..times.len()){
        if times[i].0 - times[i-1].1 > durata {
            schedule.push((times[i].0, times[i].1));
        }
    }


    schedule
}

fn main() { //cargo run cal1.txt cal2.txt 30

    let mut bounds: (u32,u32) = (0,0);
    let mut interval: (u32, u32) = (0, 0);
    let mut schedule: Vec<(u32,u32)> = Vec::new();

        //linea di comando
    let args: Vec<String> = args().skip(1).collect();

        //apri file
    let cal1 = File::open( &args[0] ).unwrap();
    let cal2 = File::open( &args[1] ).unwrap();

    //lettura e conversione terzo valore di args
    let interval = match args[2].parse::<u32>(){
        Ok(value)=>value,
        _=>panic!("errore numero minuti")
    };
    let calendar1 = Calendar::from_File(cal1);
    let calendar2 = Calendar::from_File(cal2);

    let buchi = buchi(interval, calendar1.schedule);
   // let result = buchi(interval, )

}

/*
1 ITERAZIONE
inizio = bound iniziale max
fine = tupla.0 minimo tra le tuple dei due calendari

2 ITERAZIONE
inizio =
fine =
*/