struct Point <T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point {x:5 , y:10};
    let float = Point {x:8.0,y:7.7};
    //let wontwork = Point {x:8, y:9.0};
}