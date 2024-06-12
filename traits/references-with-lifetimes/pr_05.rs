//Lifetime Annotations in Function Signatures

fn longest<'a>(x:&'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() {
        x
    } else {
        y
    }
} //The function signature now tells Rust that for some lifetime 'a, the function takes two parameters, both of which are string slices that live at least as long as lifetime 'a. The function signature also tells Rust that the string slice returned from the function will live at least as long as lifetime 'a.

fn main() {
    let string1 = String::from("jhgfjfjkdsfkdshfkdsjhfskdjf");
    let string2 = String::from("sdkjgfdsjkf");

    let longest = longest(&string1, &string2);
    println!("The longest string is {}", longest);

    // let string4 =  String::from("jhgfjfjtrrkdsfkdshfkdsjhfskdjf");
    // {
    //     let string5 = String::from("sdkjgfdsjkf");
    //     let result = longest(&string4, &string5);
    //     println!("The longest string is {}", result);
    // }
}


fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
} //this won't compile