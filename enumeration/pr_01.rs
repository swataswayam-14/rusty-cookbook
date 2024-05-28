enum IpAddrKind{
    V4,
    V6
}

struct IpAddr{
    kind:IpAddrKind,
    home:String
}

fn main(){
    let home = IpAddr{
        kind:IpAddrKind::V4,
        home:String::from("127.0.0.1"),
    };

    let loopback = IpAddr{
        kind: IpAddrKind::V6,
        home: String::from("::1"),
    };
}