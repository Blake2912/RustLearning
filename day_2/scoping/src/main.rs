use std::io;

fn main() {
    let a = [1,2,3,4,5,6,7];
    println!("Please enter an array index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to readline");

    let index: usize = index.trim().parse().expect("Parse err");
    
    let element = a[index];
    println!("The element at the given index {index} is: {}",element);
}
