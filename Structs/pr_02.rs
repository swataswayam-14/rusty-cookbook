//Using Tuple Structs without named fileds to create different types

//tuple structs

struct Color(i32, i32, i32);
struct Point(i32, i32,i32);
struct Marks(f64, i32 , i64);

fn main(){
    let black = Color(0,0,0);
    let white = color(255,255,255);
    let origin = Point(0,0,0);

    //Note that the black and origin values are different types because they’re instances of different tuple structs. Each struct you define is its own type, even though the fields within the struct might have the same types. 
    // For example, a function that takes a parameter of type Color cannot take a Point as an argument, even though both types are made up of three i32 values.
}