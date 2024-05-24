//ownership and functions

fn main(){
    let x = String::from("hello world");
    takes_ownership(x); //x's value is moved into the function , hence its no longer valid here
//If we tried to use x after the call to takes_ownership, Rust would throw a compile-time error.
    let i = 5;

    makes_copy(5); // x would move into the function,
    // but i32 is Copy, so it's okay to still
    // use x afterward
}
fn takes_ownership(some_str: String){
    println!("{}", some_str);
}// Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_int: i32){
    println!("{}", some_int);
}// Here, some_integer goes out of scope. Nothing special happens