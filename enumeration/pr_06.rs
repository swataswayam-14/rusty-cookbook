//Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent. This enum is Option<T>, and it is defined by the standard library as follows:

enum Option<T>{ //<T> : generic type parameter
    None, 
    Some(T),
}
//Option enum is included in the prelude , we donot need to bring it into scope explicitly
//Its variants are also included in the prelude: you can use Some and None directly without the Option:: prefix.

fn main(){
    let some_num = Some(5);
    let some_char = Some('w');
    let absent_number = Option<i32> = None;

    // let x:i8 = 5;
    // let y: Option<i8> = Some(5);  -> this code won't compile
    // let sum = x+y;
}

//https://doc.rust-lang.org/std/option/enum.Option.html