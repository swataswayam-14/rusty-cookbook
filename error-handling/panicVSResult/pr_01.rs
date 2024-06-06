//Cases in Which You Have More Information Than the Compiler

use std::net::IpAddr;

fn main(){
    let home:IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
}