#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32
}


fn main(){
    let width = 20;
    let height = 50;
    
    let area = calculate_area(width, height);
    println!("The area of rectangle is {}", area);

    let rect1 = (20,20);
    println!("The area of rectangle1 is {}", calculate_area1(rect1));

    let rect2 = Rectangle{
        width:30,
        height:200
    };
    println!("The area of reactangle2 is {}", calculate_area2(&rect2));
    println!("rect1 is {:?}", rect2);

    let scale = 2;
    let rect3 = Rectangle{
        width:dbg!(30*scale),
        height:50,
    };
    dbg!(&rect3);
    //We can put dbg! around the expression 30 * scale and, because dbg! returns ownership of the expression’s value, the width field will get the same value as if we didn’t have the dbg! call there. We don’t want dbg! to take ownership of rect1, so we use a reference to rect1 in the next call.
}

fn calculate_area(width:u32, height:u32)->u32{
    width * height
}

fn calculate_area1(dimensions:(u32, u32))->u32{
    dimensions.0 * dimensions.1
}

fn calculate_area2(rectangle: &Rectangle)->u32{
    rectangle.width * rectangle.height
}