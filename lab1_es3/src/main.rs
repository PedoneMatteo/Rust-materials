use std::char;
use std::cmp::min_by;
use std::env;

fn count_mines(minefield: &[&str], i:usize, j:usize)->u32{
    let first_row;
    let first_col;
    let last_row;
    let last_col;
    let mut count=0;
    //definisco quali sono i margini delle matrice da controllare per trovare le bombe attorno allo spazio [i][j]
    if i==0{ first_row=0; } else { first_row=i-1; }
    if j==0{first_col=0}else{first_col=j-1;}
    if (i+1)<minefield.len(){last_row=i+1;}else{last_row=i;}
    if (j+1) < minefield[i].len(){last_col=j+1;}else{last_col=j;}


    for k in first_row..=last_row{

        for z in first_col..=last_col{

            if minefield[k].as_bytes()[z] == '*' as u8{
                count+=1;
            }

        }
    }
    count
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    //unimplemented!("\nAnnotate each square of the given minefield with the number of mines that surround said square (blank if there are no surrounding mines):\n{minefield:#?}\n");
    let mut res = Vec::new();

    for i in 0..minefield.len(){
        let row = minefield[i];
        let mut new_row = String::new();

        for j in 0..row.len(){

            if row.as_bytes()[j]!= '*' as u8{
                let count = count_mines(minefield, i, j);
                if count==0{
                    new_row.push(' ');
                }else{
                    new_row.push(char::from_digit(count, 10).unwrap());
                }
            }else{
                new_row.push('*');
            }
        }
        res.push(new_row);
    }
    res
}

fn count_mines2(v: Vec<u8>, i:usize, j:usize, rows:usize, cols:usize)->u32{
    let first_row;
    let first_col;
    let last_row;
    let last_col;
    let mut count=0;

    //definisco quali sono i margini delle matrice da controllare per trovare le bombe attorno allo spazio [i][j]
    if i==0{ first_row=0; } else { first_row=i-1; }
    if j==0{first_col=0}else{first_col=j-1;}
    if (i+1)<rows {last_row=i+1;}else{last_row=i;}
    if (j+1)<cols {last_col=j+1;}else{last_col=j;}


    for k in first_row..=last_row{

        for z in first_col..=last_col{

            if v[k*cols+z] == '*' as u8{
                count+=1;
            }

        }
    }
    count
}

fn annotate2(minefield: String, rows: usize, cols: usize) -> String {
    let v=minefield.into_bytes();
    let mut res = String::new();
    for i in 0..rows{

        let line = &v[i*cols..i*cols+cols];  //CREO UNA SLICE

        for j in 0..cols{
            let ciao=line[j];
            if line[j]!= '*' as u8{
                let count = count_mines2(v.clone(), i, j, rows, cols);  //SENZA IL .clone() ACCADREBBE UN MOVIMENTO
                if count==0{
                    res.push('0');
                }else{
                    res.push(char::from_digit(count, 10).unwrap());
                }
            }else{
                res.push('*');
            }
        }
    }
    res
}

fn main() {
    let minefield = String::from("*  *   *  **   ");
    let s = annotate2(minefield, 3, 5);
    println!("{}", s);
/*
        //  DA LINEA DI COMANDO CON cargo run -- --rows=3 --cols=3 "* * * "
    let args: Vec<String> = env::args().collect();
    annotate(&[args[3].as_str()]);
    */

}
/*
(&str).as_bytes gives you a view of a string as a &[u8] byte slice
(that can be called on String since that defers to str, and there's also String.into_bytes
will consume a String to give you a Vec<u8>
*/