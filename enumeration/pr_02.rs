//representing the same concept using just an enum is more concise: rather than an enum inside a struct, we can put data directly into each enum variant. This new definition of the IpAddr enum says that both V4 and V6 variants will have associated String values

enum IpAddr{
    V4(String),
    V6(String),
}

fn main(){
    let home = IpAddr::V4(String::from("127.0.0.1")); //IpAddr::V4() is a function call that takes a String argument and returns an instance of the IpAddr type
    let loopback = IpAddr::V6(String::from("::1"));
}