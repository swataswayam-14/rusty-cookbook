fn main(){
    let s = "hello";
    //strings are immutable, not every string value can be known when we write the code , for ex: when we expect a string input from the user

    let mut ss = String::from("hello"); // stored in heap , this can be mutated

    ss.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", ss); // This will print `hello, world!`

    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    } // this scope is now over, and s is no
    // longer valid

    let x = 5;
    let y = x; //these two 5 values are pushed onto the stack.

    println!("x = {}, y = {}", x, y); //valid



    let s1 = String::from("hello");
    let s2 = s1; //not the case in Strings

    //println!("{}, world!", s1); - error 

    //concept of copying the pointer, length, and capacity without copying the data probably sounds like making a shallow copy. But because Rust also invalidates the first variable, instead of being called a shallow copy, it’s known as a move. In this example, we would say that s1 was moved into s2


    let s3 = String::from("hello");
    let s4 = s3.clone();//the heap data does get copied.

    println!("s3 = {}, s4 = {}", s3, s4); 
}