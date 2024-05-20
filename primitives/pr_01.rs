fn main(){
    let logical:bool = true;

    let a_float: f64 = 1.0; //Regular expression
    let an_integer = 5i32; //suffix annotation

    let default_float = 3.0; //f64
    let default_integer = 7; //i32

    let mut inferred_type = 12;
    inferred_type = 342312342321i64;

    let mut mutable = 12;
    mutable = 212;

    //mutable = true; //error : the type of variable cannot be changed

    //variables can be overwritten with shadowing

    let mutable = true;
}