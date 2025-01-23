fn main() {
    fn function (i:i32) -> i32 {i+1}

    let closure_annotated = |i:i32| -> i32 {i+1};
    let closure_inferred = |i| i+1;
    let i = 1;
    println!("Function:{}", function(i));
    println!("Closure_annotated: {}", closure_annotated(i));
    println!("Closure_inferred: {}", closure_inferred(i));

    let one = || 1;
    println!("Closure returning one: {}",one());
}