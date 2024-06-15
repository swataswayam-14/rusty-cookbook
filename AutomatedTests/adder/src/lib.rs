pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub fn mul(left: usize, right: usize) -> usize {
    left * right
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller(){
        let smaller = Rectangle {
            width: 2,
            height: 3
        };
        let larger = Rectangle {
            width: 7,
            height: 8
        };

        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn larger_can_hold_smaller2(){
        let smaller = Rectangle {
            width: 2,
            height: 3
        };
        let larger = Rectangle {
            width: 7,
            height: 8
        };
        assert!(smaller.can_hold(&larger));
    }
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn another(){
        panic!("This test is gonna fail");
    }
    #[test]
    fn test2(){
        let result = add(5,6);
        assert_eq!(result, 11);
    }
    #[test] 
    fn test3(){
        let result = mul(2,3);
        assert_eq!(result, 6);
    }
}
