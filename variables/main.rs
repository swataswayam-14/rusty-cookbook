fn main() {
    let mut x = 5; //a variable is declared as mutable , hence it can be changed and doesnot give errors
    // let x = 5; //immutable variable x
    println!("The value of x is: {x}");
    x = 6; // error, reassignment of immutable variable 
    println!("The value of x is: {x}");

    //shadowing the variable x
}
