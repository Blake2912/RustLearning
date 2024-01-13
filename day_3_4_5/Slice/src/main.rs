fn main() {
    let mut s = String::from("Hello World");

    let slice = &s[0..2];
    println!("Sliced string: {slice}");

    let slice = &s[2..];
    println!("Sliced string: {slice}");

    let full_str = &s[..];
    println!("Full string: {full_str}");

    let word = first_word(&s);

    println!("First word: {word}");

    s.clear();
    
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}