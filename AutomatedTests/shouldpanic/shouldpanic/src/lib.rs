pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new (value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}");
        }
        Guess {
            value: value  // or Guess{value}  : both are same
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100(){
        Guess::new(900);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2+2 == 4 {
            Ok(())
        }else {
            Err(String::from("two plus two does not equal four"))
        }
    }
    
}
