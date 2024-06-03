//Unlike the built-in array and tuple types, the data these collections point to is stored on the heap, which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs. 


fn main(){
    let v1: Vec<i32> = Vec::new();

    let v2 = vec![1,2,3];

    //updating a vector

    let mut v3 = Vec::new();

    v3.push(5);
    v3.push(6);
    v3.push(2324);
    v3.push(435);

    //reading elements of vectors

    let third: &i32 = &v3[2];
    println!("The third element in vector v3 is {}", third);

    let fourth: Option<&i32> = v3.get(3);
    match fourth {
        Some(fourth) => println!("The fourth element is {fourth}"),
        None => println!("There is no fourth element"),
    }

    //let does_not_exist = &v[100]; -> program panics and crashes
    let does_not_exist = v3.get(100); // program doesnot panics as get method returns None



//BELOW PROGRAM GIVES AN ERROR 
//AS HERE WE hold an immutable reference to the first element in a vector and try to add an element to the end.

//vectors put the values next to each other in memory, adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, if there isn’t enough room to put all the elements next to each other where the vector is currently stored. In that case, the reference to the first element would be pointing to deallocated memory.

    // let mut v = vec![1,2,3,4];

    // let first:&i32 = &v[0];

    // v.push(9);

    // println!("The first element is {}", first);
}