use std::io;

fn main() {
    println!("Simple calculator");

    loop{
        let mut choice = String::new();
        print_options();
        io::stdin()
            .read_line(&mut choice)
            .expect("Reading options failed");

        let choice: i32 = choice.trim().parse().expect("Parse error");
        let ret = calculate(choice);

        match ret {
            -999 => {
                match choice{
                    0 => { println!("Exiting the program"); break; }
                    _ => println!("Please enter the correct option")
                }
            }
            _ => println!("Result of calculation is {ret}")
        }
    }


}


fn print_options(){
    println!("The calculator has the following options:
    \n\t1. Addition
    \n\t2. Subtraction
    \n\t3. Multiplication
    \n\t0. Exit");
    println!("Please select one option");
}

fn calculate(choice: i32) -> i32{
    match choice{
        1 => {
            println!("You have chosen addition, please enter two numbers");
            let ip = accept_user_ip();
            return add(ip.0, ip.1);
        }
        2 => {
            println!("You have chosen subtraction, please enter two numbers");
            let ip = accept_user_ip();
            return subtract(ip.0, ip.1);
        },
        3 => {
            println!("You have chosen to multiply, please enter two numbers");
            let ip = accept_user_ip();
            return multiply(ip.0, ip.1);
        }
        0 => {
            return -999;
        }
        _ => {
            println!("Please enter a valid choice");
            return -999;
        }
    }
}

fn accept_user_ip() -> (i32, i32){
    let mut x = String::new();
    let mut y = String::new();

    io::stdin()
        .read_line(&mut x)
        .expect("Reading x failed");

    io::stdin()
        .read_line(&mut y)
        .expect("Reading y failed");

    let x: i32 = x.trim().parse().expect("Read x failed");
    let y: i32 = y.trim().parse().expect("Read y failed");

    return (x, y);
}

fn multiply(x: i32, y: i32) -> i32{
    return x * y;
}

fn subtract(x: i32, y: i32) -> i32{
    return x - y;
}

fn add(x:i32, y:i32) -> i32{
    return x + y;
}
