fn main(){
    let a = [10,20,30,40,50];

    let mut index = 0;

    while index < 5{

        //error prone, also slow , because the compiler adds runtime code to perform the conditional check of wether the index is within the bounds of the array on every iteration through the loop

        println!("{0}th item is {1}", index , a[index]);
        index += 1;
    }

    for element in a{

        //increased the safety of the code and eliminated the chance of bugs 

        println!("The value is: {element}");
    }

    //the Range : provided by the standard library

    for number in (1..4).rev(){
        println!("{number}");
    }
    

}