use std::env::args;

fn capitalize(s: &str) -> String {
    let mut check = 1;
    let mut new_s = "".to_string();

    for c in s.chars(){
        match check{
            1 => {
                let cc =c.to_ascii_uppercase();
                new_s.push(cc);
                if c != ' ' {
                    check = 0;
                }
            },
            0 => {
                if c == ' '{
                    check=1;
                }
                let cc =c;
                new_s.push(cc);
            },
            _ => panic!(),
        }
    }
    return new_s;
}

fn main() {
    let args: Vec<String> = args().skip(1).collect();
    let mut s2  = "".to_string();

    if args.len()>0{
        println!("we have args!\n");
    }
     for ss in args{
         s2.push_str(ss.as_str());
         s2.push(' ');
     }
    let s_def = capitalize(s2.as_str());

    /* let s1 = "trasforma in   maiuscolo le iniziali di queste parole";
    let s_def = capitalize(s); */

    println!("{}", s_def);
}
