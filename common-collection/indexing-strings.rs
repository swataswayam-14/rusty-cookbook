//Indexing into Strings

fn main(){
    let s = String::from("hello");
    // let h = s1[0]; :  error
    let hello = String::from("Hola");
    let hello1 = String::from("Здравствуйте");

    //string slicing
    let s = &hello1[0..4];

    //iterating over strings

    for c in "Зд".chars(){
        println!("{c}");
    }
    for c in "Зд".bytes(){
        println!("{c}");
    }

}