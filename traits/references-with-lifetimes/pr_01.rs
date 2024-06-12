fn main(){
    let r;

    {
        let x = 5;
        r = &x;
    }
    println!("r: {}", r);
}
//The outer scope declares a variable named r with no initial value, and the inner scope declares a variable named x with the initial value of 5. Inside the inner scope, we attempt to set the value of r as a reference to x. Then the inner scope ends, and we attempt to print the value in r. This code won’t compile because what the value r is referring to has gone out of scope before we try to use it.