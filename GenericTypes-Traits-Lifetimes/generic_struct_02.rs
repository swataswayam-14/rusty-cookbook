struct Point <T, U> {
    x: T,
    y: U,
}

fn main(){
    let both_int = Point {x:1, y:2};
    let both_float = Point {x:1.2, y:5.2};
    let int_float = Point {x:1.34, y:9};
}