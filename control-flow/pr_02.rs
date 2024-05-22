fn main(){
    let condition:bool = false;

    let x = if condition {
        5
    }else{
        7
    };

    println!("The value of x is {x}");

    let y = if condition {4} else {9};
    println!("The value of y is {y}");


    // let z = if condition {5} else {"sdkjgf"};  the if and else arms have values that are incompatible : error

    //This won’t work because variables must have a single type, and Rust needs to know at compile time what type the z variable is.
}