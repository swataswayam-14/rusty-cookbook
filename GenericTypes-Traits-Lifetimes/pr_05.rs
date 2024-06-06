fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T { //To enable comparisons, the standard library has the std::cmp::PartialOrd trait
    //here we restrict the types valid for T to only those that implement PartialOrd and this example will compile, because the standard library implements PartialOrd on both i32 and char.
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main(){
    let list_i32 = vec![12,43,76,33,56,89,3,97,9,7,9,876,975,4,3,3,5];
    let list_char = vec!['a','f','f','b', 'm','g','h'];

    let largest_i32 = largest(&list_i32);
    let largest_char = largest(&list_char);

    println!("The largest integer is {}", largest_i32);
    println!("The largest character is {}", largest_char);
}