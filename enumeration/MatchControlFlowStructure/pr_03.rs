//Catch-all Patterns and the _ Placeholder


fn dice_roll(dice_roll: i32){
    match dice_roll{
        3 => call_function1(),
        7 => call_function2(),
        other => call_function3(other)
    }
}
//Note that we have to put the catch-all arm last because the patterns are evaluated in order. If we put the catch-all arm earlier, the other arms would never run, so Rust will warn us if we add arms after a catch-all!

fn dice_roll2(dice_roll: i32){
    match dice_roll{
        3 => call_function1(),
        7 => call_function2(),
        _ => reroll() //_ is a special pattern that matches any value and does not bind to that value. This tells Rust we aren’t going to use the value, so Rust won’t warn us about an unused variable.
    }
}

fn dice_roll3(dice_roll: i32){
    match dice_roll{
        3 => call_function1(),
        7 => call_function2(),
        _ => () //Here, we’re telling Rust explicitly that we aren’t going to use any other value that doesn’t match a pattern in an earlier arm, and we don’t want to run any code in this case.
    }
}

fn call_function1(){
    println!("Function 1 is called as the dice roll value is 3");
}
fn call_function2(){
    println!("Function 2 is called as the dice roll value is 7");
}
fn call_function3(x:i32){
    println!("Function 3 is called as the dice roll value is {}", x);
}
fn reroll(){
    println!("Dice reroll");
}

fn main(){
    dice_roll(3);
    dice_roll(7);
    dice_roll(10);
    dice_roll2(34);
    dice_roll3(23);
}