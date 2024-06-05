use std::fs::File;

fn main(){
    let greeting_file = File::open("hello3.txt")
        .expect("hello3.txt should be present in this directory");
}