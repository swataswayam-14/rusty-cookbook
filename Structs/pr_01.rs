struct User{
    active:bool,
    username:String,
    email:String,
    sign_in_count:u64,
}

fn main(){
    let mut user1 = User{ //Note that the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable.
        active:true,
        username:String::from("someusername123"),
        email:String::from("random@gmail.com"),
        sign_in_count:1
    };

    user1.email = String::from("another@gmail.com");
    println!("The email of user1 is {}", user1.email);

    let user2:User = build_user(String::from("swayam@gmail.com"), "swataswayam.14".to_string());
    println!("The email of user 2 is {0} and its username is {1}, sigincount: {2}", user2.email, user2.username, user2.sign_in_count);

    //creating instances from other instances with Struct update syntax

    let user3 = User{
        active:user1.active,
        username:String::from("user3hala"),
        email:String::from("user3@gmail.com"),
        sign_in_count:user1.sign_in_count,
    };

    //Using struct update syntax, we can achieve the same effect with less code
    let user4 = User{
        email:String::from("user4@gmail.com"),
        ..user1
    };

    //struct update syntax uses = like an assignment
    //we can no longer use user1 as a whole after creating user4 because the String in the username field of user1 was moved into user4.

    // println!("The username of user1 is {}", user1.username);  -> error
    println!("the username of user3 is {0} and user4 is {1}", user3.username, user4.username);

    //Both active and sign_in_count are types that implement the Copy trait
}

fn build_user(email:String , username:String) ->User{
    User{
        active:true,
        username, //field init shorthand syntax
        email, //field init shorthand syntax
        sign_in_count:1,
    }
}