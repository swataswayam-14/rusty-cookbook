//Version four IP addresses will always have four numeric components that will have values between 0 and 255. If we wanted to store V4 addresses as four u8 values but still express V6 addresses as one String value, we wouldn’t be able to with a struct. Enums handle this case with ease:

enum IpAddr{
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main(){
    let home = IpAddr::V4(127,0,0,1);
    let loopback = IpAddr::V6(String::from("::1"));
}