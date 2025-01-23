#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor:: Red => num_red +=1,
                ShirtColor::Blue => num_blue += 1,
            }

        }
        if num_red > num_blue {
            ShirtColor::Red 
        }else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec! [ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };
    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}", user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}", user_pref2, giveaway2
    );
}

//difference between the syntax of function and closure

fn add_one(i:u32) -> u32 { i+ 1 }
let add_one_v1 |i:u32| -> u32 {i+1};
let add_one_v2 |i| {i+1};
let add_one_v3 |i| i+1;


let example_closure = |x| x ///closure which returns any value which it is passed to


//Attempting to call a closure whose types are inferred with two different types

//example usage
let s = example_closure(String::from("hello"));
let s2 = example_closure(3);//The first time we call example_closure with the String value, the compiler infers the type of x and the return type of the closure to be String. Those types are then locked into the closure in example_closure, and we get a type error when we next try to use a different type with the same closure.