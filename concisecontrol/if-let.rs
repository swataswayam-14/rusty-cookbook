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


fn main(){
    let config_max = Some(3u8);

    match config_max{
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    let using_if_let = Some(4u8);
    if let Some(max) = using_if_let{
        println!("The maximum is configured to be {}", max);
    }

    let mut count = 0;

    let coin = Coin::Quarter(IndiaState::Odisha);
    let coin2 = Coin::Quarter(IndiaState::Kashmir);

    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
            _ => count += 1,

        }
        if let Coin::Quarter(state) = coin2{
            println!("State quarter from {:?}!", state);
        }else{
            count += 1
        }
    }

    

