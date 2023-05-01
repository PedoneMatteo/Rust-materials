use std::char;
use std::env::args;
use std::fs;
use std::fs::{File};
use std::io::{Read};

/*struct ValueStruct{
    tipo: i32,
    val: f32,
    timestamp: i64,
}
struct MValueStruct{
    tipo: i32,
    val: [f32; 10],
    timestamp: i64,
}
struct MessageStruct{
    tipo: i32,
    message: [char; 21],
}

enum CData{
    VS {data: ValueStruct},
    MVS {mdata: MValueStruct},
    MessS {mess: MessageStruct},
}*/

struct  CData {
    t: i32,
    val: Vec<f32>,
    timestamp: i64,
    message: Vec<char>,
}



impl CData {
    fn from_file(&mut self, f1: &mut File){
        let mut buffer = [0;4];     //buffer da 4 byte

        f1.read_exact(&mut buffer);

        let tipo = i32::from_be_bytes(buffer);


        match tipo{
            1 => {
                let mut val1 = [0;4];
                let mut val2 = [0;8];
                f1.read_exact(&mut val1);
                f1.read_exact(&mut val2);

                self.t = tipo;
                self.val.push(f32::from_be_bytes(val1));
                self.timestamp = i64::from_be_bytes(val2);
                return;
            }

            2 => {
                let mut val1 = [0;4];
                let mut val2 = [0;8];
                let mut vec_val: Vec<f32> = Vec::new();
                for i in [0..10]{
                    f1.read_exact(&mut val1);
                    vec_val.push(f32::from_be_bytes(val1));
                }
                f1.read_exact(&mut val2);

                self.t = tipo;
                self.val = vec_val;
                self.timestamp = i64::from_be_bytes(val2);

                return;
            }

            3 => {
                let mut val1= [0;1];
                let mut vec_val: Vec<char> = Vec::new();
                for i in [0..21]{
                    f1.read_exact(&mut val1);
                    vec_val.push(char::from(val1[0]));
                }

                self.t = tipo;
                self.message= vec_val;
                return;
            }
            _ => {}
        }
    }
}

fn main() {
    let mut values: Vec<CData> = Vec::new();
    let args: Vec<String> = args().skip(1).collect();
    let mut file = fs::read("data.bin");
    match file {
        Ok(data)=>{
            for i in 0..100{
                values[i].from_file(&mut data);
            }
        },
        Err(e)=>{
            panic!("errore durante la lettura del file");
        }
    }
       //let mut file = File::open("data.bin").expect("Impossibile leggere il file");
    //   for i in 0..100{
    //     values[i].from_file(&mut data);
    //   }

    println!("Hello, world!");
}
