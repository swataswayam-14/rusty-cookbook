fn main(){
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    //let s3:String = takes_and_gives_back(s2);

    //error as s2 cannot be printed as it is moved to the function takes_and_gives_back

    let s3:String = takes_and_gives_back(s2.clone());


    println!("s1 = {0}, s2= {1}, s3 = {2}", s1, s2, s3);

    let s5 = String::from("hello");

    let (s4 , len) = calculte_length(s5);
    println!("The length of '{}' is {}.", s4, len);



}

fn takes_and_gives_back(a_string:String)->String{
    a_string
}
fn gives_ownership()->String{
    let some_str = String::from("hey you");

    some_str
}

fn calculte_length(s:String) -> (String , usize){
    let length = s.len();

    (s, length)
}