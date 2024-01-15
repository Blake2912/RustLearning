fn main() {

    let data = "initial contents";
    let mut s = data.to_string();

    println!("{s}\n");
    // Other way
    // let s = "Some String".to_string();
    let hello = String::from("السلام عليكم");
    println!("{hello}");
    let hello = String::from("Dobrý den");
    println!("{hello}");
    let hello = String::from("Hello");
    println!("{hello}");
    let hello = String::from("שָׁלוֹם");
    println!("{hello}");
    let hello = String::from("नमस्ते");
    println!("{hello}");
    let hello = String::from("こんにちは");
    println!("{hello}");
    let hello = String::from("안녕하세요");
    println!("{hello}");
    let hello = String::from("你好");
    println!("{hello}");
    let hello = String::from("Olá");
    println!("{hello}");
    let hello = String::from("Здравствуйте");
    println!("{hello}");
    let hello = String::from("Hola");
    println!("{}", hello);

    // Updating a string
    s.push_str(" Pushed str");
    println!("Pushed string result: \n{s}");

    // String concatenation
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let concatenated_string = format!("{s1}-{s2}-{s3}");

    println!("{concatenated_string}");

    // Slicing and accessing via index
    let another_string = "tic-tac-toe"; 
    // '&str' is stored as ['t', 'i', 'c', '-', ...] so accesing via index is possible, but not with 'String'

    let tic = &another_string[0..3];
    println!("{}",tic);

    // Converting to bytes, and chars
    let bytes_str = "hEllO WoRld";

    // Chars
    for i in bytes_str.chars(){
        println!("char: {i}");
    }
    println!("\n");
    // Bytes
    for i in bytes_str.bytes(){
        println!("bytes: {i}");
    }

    

}
