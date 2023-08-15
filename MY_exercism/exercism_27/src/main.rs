//Convert a phrase to its acronym.
// Techies love their TLA (Three Letter Acronyms)!
// Help generate some jargon by writing a program that converts a long name like Portable Network Graphics to its acronym (PNG).
pub fn abbreviate(phrase: &str) -> String {
    phrase.split(" ")
        .map(|word| word.chars().filter(|x| {
            x.is_ascii_uppercase()
        } )
            .next()
            .unwrap()).collect::<String>()
}

fn main() {
    let s = abbreviate("Portable Network Graphics");
    println!("{}", s);
}
