use std::fs;
use std::io;

fn read_username_from_file2() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn main(){
    match read_username_from_file2(){
        Ok(username) => println!("{}", username),
        Err(e) => println!("Unable to read username from the file : {}", e),
    }
}