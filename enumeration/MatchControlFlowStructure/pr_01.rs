//write a function that takes an unknown US coin and, in a similar way as the counting machine, determines which coin it is and returns its value in cents

enum Coin{
    Penny, 
    Nickel,
    Dime,
    Quarter(IndiaState),
}
#[derive(Debug)]
enum IndiaState{
    Odisha, 
    Delhi,
    Mumbai,
    Kashmir
}

//This seems very similar to a conditional expression used with if, but there’s a big difference: with if, the condition needs to evaluate to a Boolean value, but here it can be any type. The type of coin in this example is the Coin enum that we defined on the first line.

fn value_in_cents(coin:Coin)->u8{
    match coin{
        Coin::Penny => {
            println!("lucky bruh");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state)=>{
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main(){
    let value: u8 = value_in_cents(Coin::Penny);
    println!("The value is {}", value);
    let quarter = value_in_cents(Coin::Quarter(IndiaState::Odisha));
}