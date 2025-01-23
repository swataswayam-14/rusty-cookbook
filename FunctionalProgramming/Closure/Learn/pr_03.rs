fn main() {
    let mut list = vec![1,2,3];
    println!("Before creating closure: {list:?}");
    let mut borrow_mutably = || {
        list.push(7);
        println!("From closure: {list:?}")};
    borrow_mutably();
    borrow_mutably();
    println!("After calling closure: {list:?}")
}