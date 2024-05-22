fn main(){
    let num = 16;

    if num < 5{
        println!("The number is less than 5");
    }else{
        println!("The number is greater than or equal to 5");
    }

    // if num{
    //     println!("this gives an error as it expects a bool but got an integer");
    // }

    //rust will not convert non boolean values to boolean (unlike C , javaScript etc)

    if num!=0{
        println!("The number is not zero ");
    }

    if num % 4 == 0{
        println!("The number is divisible by 4 ");
    }else if num % 3 == 0{
        println!("The number is divisible by 3 ");
    }else if num % 2 == 0{
        println!("The number is divisible by 2 ");
    }else{
        println!("The number is not divisible by 2 , 3 and 4 ");
    }
}