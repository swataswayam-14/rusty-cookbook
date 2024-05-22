fn main(){
    let y = {
        let x = 4;
        x+1
    };
    println!("The value of y is {y}"); //x = x+1 doesnot have a semicolon at the end -> its a expression. adding a semicolon converts it into a statement and make it doesnot return a value
//     let z = (let a = 9); --> error : statements donot return values

}