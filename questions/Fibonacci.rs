fn main(){

    let x:i32 = findNthFib(4);
    println!("The 4th fibonacci number is {0}", x);
}

fn findNthFib(x:i32)->i32{
    if x == 0{
        0
    }else if x == 1{
        1
    }else{
        findNthFib(x-1) + findNthFib(x-2)
    }
}