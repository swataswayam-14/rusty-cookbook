fn main(){
    let num_list = vec![100,12,12,1211,456,344,2,3,4];
    let mut largest = &num_list[0];

    for num in &num_list{
        if num > largest{
            largest = num;
        }
    }
    println!("The largest number is {}", largest);
    
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}