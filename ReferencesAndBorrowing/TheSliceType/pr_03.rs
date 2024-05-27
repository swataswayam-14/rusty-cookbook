//first_word in a string

fn main(){
    let s =  String::from("hello my name is swayam");
    let firstWord = first_word(&s);
    println!("The first word in {}", firstWord);

    // s.clear();
    // println!("The first word in {}", firstWord); //compile-time error

}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}
fn first_word2(s: &str) -> &str { //it allows us to use the same function on both &String values and &str values.
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}