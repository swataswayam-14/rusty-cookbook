//generic lifetimes in function

fn longest(x:&str, y: &str) -> &str {
    if x.len() >= y.len() {
        x
    } else {
        y
    }
} //it is not possible for the compiler to determine whether the reference we return will always be valid. The borrow checker can’t determine this either, because it doesn’t know how the lifetimes of x and y relate to the lifetime of the return value.