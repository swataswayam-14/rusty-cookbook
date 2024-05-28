//Matching with Option<T>

fn plus_one(x:Option<i32>) -> Option<i32> {
    match x{
        None => None,
        Some(i) => Some(i+1),
    }
}
//Matches Are Exhaustive
//the arms’ patterns must cover all possibilities.

//example : fn plus_one(x: Option<i32>) -> Option<i32> {
    //     match x {
    //         Some(i) => Some(i + 1),
    //     }
    // } -> won't compile

fn main(){
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("Five: {:?}!", five);
    println!("Five: {:?}!", six);
}