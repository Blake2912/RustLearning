use std::{fs::File, io::{self, Read}};


fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username){
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_another() -> Result<String, io::Error>{
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username);
    Ok(username)
}

fn main() {
    let _greeting_file_result = File::open("hello.txt");

    /*
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => match File::create("hello.txt") {
                Ok (fc) => fc,
                Err(e) => panic!("Problem in creating a file: {:?}", e),
            },
            other_error => {
                panic!("Problem In opening the file: {:?}", other_error);
            }
        }
    };
    */




    let _another_greeting_file = File::open("Somepath.txt")
        .expect("Somepath.txt is not found it is included in the project!");    
}
