fn main(){
    println!("{} months", 4);


    println!("First object: {0}, second object: {1}, third object: {2}", "object1", "Object2", "Object3");

    println!("{theory} {description} {proof}" ,theory="This is a theory", description= " This is the description of the theory" ,proof=" This is its proof");


    println!("Base 10: {}", 69420);
    println!("Base 2 (binary) {:b}", 69420);
    println!("Base 8 (octal) {:0}", 69420);
    println!("Base 16 (hexadecimal): {:x}", 69420);

    println!("{number:>5}", number=1);


    println!("{number:0<5}", number=1);
    println!("{number:0>5}", number=1);


    let number:f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

}