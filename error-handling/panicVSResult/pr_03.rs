pub struct Guess {
    value: i32,
}

impl Guess{
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value should be between 1 and 100, got {}", value);
        }
        Guess {value}
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main(){
    //let guess = Guess::new(-32); -> panic
    let guess = Guess::new(32);
    println!("The value of guess is {}", guess.value)
}