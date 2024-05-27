#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32
}
//All functions defined within an impl block are called associated functions because they’re associated with the type named after the impl

impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height 
    }
}

impl Rectangle{
    fn width(&self) -> bool{
        self.width > 0
    }
}


impl Rectangle{ // Methods can take multiple parameters that we add to the signature after the self parameter, and those parameters work just like parameters in functions.
    fn can_hold(&self, other:&Rectangle)->bool{
        if (self.width > other.width) && (self.height > other.height) {
            return true; 
        }
        false
    }
}
//We can define associated functions that don’t have self as their first parameter (and thus are not methods) because they don’t need an instance of the type to work with.
impl Rectangle{ //Associated functions that aren’t methods are often used for constructors that will return a new instance of the struct.
    fn square(size:u32)->Self{
        Self{
            width:size,
            height:size
        }
    }
}

impl Rectangle{
    fn get_width(&self)->u32{
        self.width
    }
    fn get_height(&self)->u32{
        self.height
    }
    fn get_perimeter(&self)->u32{
        2 * (self.height + self.width)
    }
}

fn main(){
    let rect1 = Rectangle{
        width:30,
        height:30
    };
    println!("The area of the rectangle is {}", rect1.area());


    if rect1.width(){
        println!("The rectangle has a width whose value is greater than 0 it is {}", rect1.width);
    }

    let rect2 = Rectangle{
        width:20,
        height:210
    };
    if rect1.can_hold(&rect2) {
        println!("Rectangle 1 can hold rectangle 2");
    }else{
        println!("Rectangle 1 cannot hold rectangle 2");
    }

    let sq = Rectangle::square(40); //This function is namespaced by the struct: the :: syntax is used for both associated functions and namespaces created by modules. ex: String::from("hello");
    println!("the area of the square is {}", sq.area());

    println!("Width: {0}, Height: {1} and Perimeter: {2} ", rect1.get_width(), rect1.get_height(), rect1.get_perimeter());
}