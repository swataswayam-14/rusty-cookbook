fn find_largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for num in list {
        if num > largest {
            largest = num;
        }
    }
    largest
}

fn main(){
    let num_list1 = vec![12,213,221,85545,66,24,74,232,232,42442,35,2324];
    let num_list2 = vec![12,223,21,45,616,24,74,232,13232,42442,35,2324];
    let num_list3 = vec![11132,243,21,45,366,424,74,231312,2332,42442,35,2324];

    let largest1 = find_largest(&num_list1);
    let largest2 = find_largest(&num_list2);
    let largest3 = find_largest(&num_list3);

    println!("The largest number in list 1 is {}", largest1);
    println!("The largest number in list 2 is {}", largest2);
    println!("The largest number in list 3 is {}", largest3);
}