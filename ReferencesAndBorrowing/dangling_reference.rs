fn main(){
    let referrence_to_nothing = dangle();
}

fn dangle() -> &String{// dangle returns a reference to a String
    let s = String::from("hello");// s is a new String
    &s// we return a reference to the String, s
}
// Here, s goes out of scope, and is dropped. Its memory goes away.
//That means this reference would be pointing to an invalid String. That’s no good! Rust won’t let us do this.



//In Rust the compiler guarantees that references will never be dangling references: if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.

//the above code will give an error