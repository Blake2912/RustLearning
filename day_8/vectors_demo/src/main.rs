use std::{vec, fmt::Debug};

fn main() {
    let mut v: Vec<i32> = Vec::new();

    let mut v1: Vec<i32> = vec![1, 2, 3];

    // Inserting elements to a vector
    v.push(5);
    v.push(6);
    v.push(7);

    let immutable_borrow = &v1[0];

    println!("The immutable borrow value is {immutable_borrow}"); // --> This works as we are using the variable before a change takes place in the vector
    v1.push(8);
    // println!("The immutable borrow value is {immutable_borrow}"); --> This won't work as we are changing the V1 vector where there is an already immutable reference
    print!("\n");
    let third: Option<&i32> = v.get(2);

    match third {
        Some(third) => println!("The thrid element is {third}"),
        None => println!("No third element is present"),
    }

    print!("\n");

    // Iterating over a vector
    let v =  vec![100, 32, 57];
    for i in &v{
        println!("Value: {i}");
    }

    print!("\n");
    // Mutating the values and then again printing
    let mut some_vec =  vec![100, 32, 57];
    for i in &mut some_vec{
        println!("Pre change: {i}");
        *i += 50;
        println!("Post change:{i}");
    }

    print!("\n");
    println!("Post update of the vector");
    for i in &some_vec{
        println!("Value: {i}");
    }

    #[derive(Debug)]
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12)
    ];

    for i in &row{
        println!("\n{:#?}", i);
    }

}