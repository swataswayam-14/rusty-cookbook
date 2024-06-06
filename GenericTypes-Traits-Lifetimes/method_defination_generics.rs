struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point {x: 78, y:9};
    println!("p.x = {}", p.x());

    let v = Point {x:8.9, y:9.2};
    println!("Distance from origin of point having x : {0} and y : {1} is {2}", v.x(), v.y(), v.distance_from_origin());
}