fn main(){
    let s = String::from("hello my name is Swayam");

    let len = calculte_length(&s); // &s syntax lets us create a reference that refers to the value of s but does not own it. Because it does not own it, the value it points to will not be dropped when the reference stops being used.

    println!("The length of '{0}' is {1}", s , len);

    let mut s1 = String::from("hello my name is Swayam");

    change_something(&mut s1);
    println!("s1 is {}", s1);

    //change_something(&s); -> we cannot modify something we are borrowing


    // if you have a mutable reference to a value, you can have no other references to that value. This code that attempts to create two mutable references to s will fail:
    
    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);

//Data races cause undefined behavior and can be difficult to diagnose and fix when you’re trying to track them down at runtime; Rust prevents this problem by refusing to compile code with data races!

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;


    //We also cannot have a mutable reference while we have an immutable one to the same value.
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM

    // println!("{}, {}, and {}", r1, r2, r3);





    //below code will compile because the last usage of the immutable references, the println!, occurs before the mutable reference is introduced
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
//The scopes of the immutable references r1 and r2 end after the println! where they are last used, which is before the mutable reference r3 is created. These scopes don’t overlap, so this code is allowed
}



fn calculte_length(s:&String)->usize{ //function uses & to indicate that the type of the parameter s is a reference.
    s.len()
}
// Here, s goes out of scope. But because it does not have ownership of what it refers to, it is not dropped.


//creating a reference is termed as borrowing


// fn change_something(some_str: &String){
//     some_str.push_str(",  aka paplu");
// }

//fixing this:

fn change_something(some_str: &mut String){ //Mutable References
    some_str.push_str(",  aka paplu");
}