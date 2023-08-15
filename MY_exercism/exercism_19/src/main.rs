//Given a number n, determine what the nth prime is.
// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
// If your language provides methods in the standard library to deal with prime numbers, pretend they don't exist and implement them yourself.
// Remember that while people commonly count with 1-based indexing (i.e. "the 6th prime is 13"), many programming languages,
// including Rust, use 0-based indexing (i.e. primes[5] == 13). Use 0-based indexing for your implementation.

pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![];
    (2..).filter(|candidate: &u32| {
            if !primes.iter().any(|j| { println!("candidate % i : {} % {} ={:?}",candidate, j,candidate % j ); candidate % j == 0 }) {
                primes.push(*candidate);
                true
            } else {
                false
            }
        })
        .nth(n as usize)
        .unwrap()
}

fn main() {
    let n:u32 = 5;
    let numb = nth(n);
    println!("The prime number is {}", numb);
}
