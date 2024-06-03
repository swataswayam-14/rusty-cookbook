fn main(){
    let v = vec![100,121,698];
//for loop to get immutable references to each element in the vector
    for i in &v{
        println!("{i}");
    }

    let mut v1 = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);
//We can also iterate over mutable references to each element in a mutable vector in order to make changes to all the elements

    for i in &mut v1{
        *i += 50; //To change the value that the mutable reference refers to, we have to use the * dereference operator to get to the value in i before we can use the += operator
    }

    for i in &v1{
        println!("{i}");
    }
}