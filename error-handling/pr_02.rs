//RECOVERABLE ERRORS:
// enum Result<T, E>{
//     Ok(T),
//     Err(E),
// }
use std::fs::File;

fn main(){
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result{
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}