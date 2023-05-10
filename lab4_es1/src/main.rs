use itertools::Itertools;
use std::thread;
use std::time::Instant;

enum op {
    Add,
    Sub,
    Mul,
    Div,
}

fn permute_with_repetition<T: Clone>(nums: &[T], length: usize, current: &mut Vec<T>, result: &mut Vec<Vec<T>>) {
    if current.len() == length {
        result.push(current.clone());
        return;
    }

    for num in nums {
        current.push(num.clone());
        permute_with_repetition(nums, length, current, result);
        current.pop();
    }
}


fn all_solutions(permutations_nums: Vec<Vec<i32>>, tid: i32){
    let vec_operations = vec![1,2,3,4]; //vettore di interi
    let length = 4;

    let tID = thread::current().id();   //id del thread

    let mut perm_operations = Vec::new();
    let mut current = Vec::new();

    permute_with_repetition(&vec_operations, length, &mut current, &mut perm_operations);   //genero tutte le possibili combinazioni di operazioni

    let mut cont = 0;
    for perm in permutations_nums {
        for ops in perm_operations.iter(){

            let mut result = perm[0];

            for (i, operation) in ops.iter().enumerate(){

                match operation{
                    1 => result += perm[i + 1],
                    2 => result -= perm[i + 1],
                    3 => result *= perm[i + 1],
                    4 => result /= perm[i + 1],
                    _ => (),
                }
            }
            if result == 10 {
                cont+=1;
                println!("[{:?}] -> {} {} {} {} {} = 10", tID, perm[0], perm[1], perm[2], perm[3], perm[4]);
            }
        }
    }
}

fn main() {         //iter da un iteratore che torna referenze, into_iter torna un iteratore su valori e non referenze
    let start = Instant::now();

    let nums = vec![2, 7, 2, 2, 1];
    let nums_len = nums.len();
    let perm_nums :Vec<Vec<i32>> = nums.into_iter().permutations(nums_len).collect();

    let mut tid = 1;
    let mut vec_threads = Vec::new();

    for chunck in perm_nums.chunks(perm_nums.len()/2){
        let part_of_perm = chunck.to_owned();

        let tidtoshare=tid.clone(); //lo clono prima del thread e non al suo interno perchè altrimenti il thread ne prende possesso e quindi clonarlo non ha senso
        //perchè clonerei qualcosa di cui non ho più il possesso

        let t = thread::spawn(move || {all_solutions(part_of_perm, tidtoshare);});
        //scrivo la move perchè cosi il thread prende il possesso vero e proprio di part_of_perm e tidtoshare
        vec_threads.push(t);

        tid+=1;
    }

    for t in vec_threads{
        t.join().unwrap();
    }

    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
   // let t1 = thread::spawn( || {all_solutions(&perm_nums[0..n/2], perm_operations.clone());});
    //let t2 = thread::spawn( || {all_solutions(&perm_nums[n/2..], perm_operations.clone());});

//    t1.join().unwrap();
 //   t2.join().unwrap();
}
