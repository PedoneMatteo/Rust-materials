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
        Self { schedule, bounds }
    }
}
    fn u32_to_time(timestamp: u32) -> String {
        format!("{:02}:{:02}", timestamp / 60, timestamp % 60)
    }


fn main() { //cargo run cal1.txt cal2.txt 30

    let mut bounds: (u32,u32) = (0,0);
    let mut interval: (u32, u32) = (0, 0);
    let mut schedule: Vec<(u32,u32)> = Vec::new();

        //linea di comando
    let args: Vec<String> = args().skip(1).collect();

        //apri file
    /*  DA LINEA DI COMANDO SI FA COSÌ:
    let cal1 = File::open( &args[0] ).unwrap();
    let cal2 = File::open( &args[1] ).unwrap();

        //lettura e conversione terzo valore di args
     let interval = match args[2].parse::<u32>(){
        Ok(value)=>value,
        _=>panic!("errore numero minuti")
     };
     */

    let file1 = "cal1.txt".to_string();
    let file2 = "cal2.txt".to_string();

    let cal1 = File::open( &file1).unwrap();
    let cal2 = File::open( &file2).unwrap();

    let min_duration = 30;

    let calendar1 = Calendar::from_File(cal1);
    let calendar2 = Calendar::from_File(cal2);

   // let buchi = buchi(interval, calendar1.schedule);
   // let result = buchi(interval, )
    println!("{:?}", calendar1);
    println!("{:?}", calendar2);

    let mut iter1 = calendar1.schedule.iter().peekable();
    let mut iter2 = calendar2.schedule.iter().peekable();
    let mut start = calendar1.bounds.0.max(calendar2.bounds.0);
    let end = calendar1.bounds.1.min(calendar2.bounds.1);
    
    while iter1.peek().is_some() && iter2.peek().is_some() {

        let x1 = iter1.peek().unwrap();
        let x2 = iter2.peek().unwrap();
        if x1.0 < start + min_duration {
            start = x1.1;
            iter1.next();
        } else if x2.0 < start + min_duration {
            start = x2.1;
            iter2.next();
        } else {
            let tmp = x1.0.min(x2.0);
            println!("{} - {}", u32_to_time(start), u32_to_time(tmp));
            start = x1.1.max(x2.1);
            iter1.next();
            iter2.next();
        }

    }

        if iter1.peek().is_none() && iter2.peek().is_none() {
            if start + min_duration <= end {
                println!("{} - {}", u32_to_time(start), u32_to_time(end));
            }
        } else {
            let mut iter = if iter1.peek().is_none() {iter2} else {iter1};
            while iter.peek().is_some() {
                let x = iter.peek().unwrap();
                let bound = x.0.min(end);
            //println!("{} | {} | {}", u32_to_time(bound), u32_to_time(start), u32_to_time(start+min_duration));
                if bound >= start + min_duration {
                    println!("{} - {}", u32_to_time(start), u32_to_time(bound));
                    start = x.1;
                }
                iter.next();
            }
            if start+min_duration <= end {
                println!("{} - {}", u32_to_time(start), u32_to_time(end));
            }
        }
}