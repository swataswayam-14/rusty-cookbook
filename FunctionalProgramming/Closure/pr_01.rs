fn main(){
    let x = 1;
    let closure = |val| val + x;
    assert_eq!(closure(2), 3);//Unlike functions, both the input and return types of a closure can be inferred by the compiler.
}