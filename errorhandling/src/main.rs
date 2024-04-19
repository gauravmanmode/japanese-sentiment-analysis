use std::fs::{self, File};
use std::io::{self, Read};

fn main() {
    // let greeting_file_result = File::open("a.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("a.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error);
    //         }

    //     },
    // };

   // let greeting_file = File::open("b.txt").expect("No such file or directory");
    
    match read_username_from_file2() {
        Ok(a) => println!("{a}"),
        Err(a) => panic!(),
    };

    let a = last_char_of_first_line("hello").expect("some thing");
    println!("{a}");
}



fn read_username_from_file() -> Result<String, io::Error> {

    let mut username = String::new();
    File::open("a.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file2() -> Result<String, io::Error> {

   fs::read_to_string("a.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}


