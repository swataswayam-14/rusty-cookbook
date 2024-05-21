fn main(){

    //TUPLE
    let tup: (i32, f64, bool) = (121,123.21,true);
    let tup2 = (213,123,532);

    let (x,y,z) = tup2;

    println!("The value of y is: {y}");

    let x: (i32, f32, bool) = (123,213.43, false);
    let first = x.0;
    let second = x.1;
    let third = x.2;
    println!("The first value in the tuple x is {first}");
    println!("The second value in the tuple x is {second}");
    println!("The third value in the tuple x is {third}");


    //ARRAY

    let a = [1,2,3,4,5,5];
    let months = ["January", "February", "March", "April", "May", "June", "July","August", "September", "October", "November", "December"];

    let afirst = a[0];
    let asecond = a[1];

    let c: [i32; 5] = [1,2,3,4,5];

    let d = [3;5]; //[3,3,3,3,3]


}