fn main(){
    println!("Hello from main function");
    another_function();
    parameter_function(5);
    print_labeled_measurements(12, 'g');

    let x = five();
    let y = plus_one(6);

    println!("The value of x is {x}");
    println!("The value of y is {y}");
}

fn another_function(){
    println!("Hello from another function");
}
fn parameter_function(x:i32){
    println!("the value of x is {x}");
}

fn print_labeled_measurements(value: i32 , unit_label: char){
    println!("The measurement is {value} {unit_label}");
}

//functions with return values

fn five() -> i32{
    5 //doesnot have a semicolon as we want it to be an expression and return 5 from where the function gets called
}

fn plus_one(x:i32)->i32{
    x+1
}