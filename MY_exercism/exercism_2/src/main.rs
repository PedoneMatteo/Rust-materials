//Reverse a string
//
// For example: input: "cool" output: "looc"
//
// Test your function on this string: uüu and see what happens. Try to write a function that properly reverses this string. Hint: grapheme clusters
pub fn reverse(input: &str) -> String {
    let mut s = Vec::with_capacity(input.len());
    for (_,c) in input.chars().enumerate(){
        s.insert(0,c);
    }
    let mut s1 = String::new();
    for c in s.iter(){
        s1.push(*c);
    }
    s1
}

fn main() {
    let input = "Hello, world!";
    let reversed = reverse(input);
    println!("Input: {}", input);
    println!("Reversed: {}", reversed);
}
