pub fn print_and_return_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}
pub fn add_two(a: u32) -> u32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    fn test1(){
        let value = print_and_return_10(9);
        assert_eq!(10, value);
    }
    //#[test]
    fn test2(){
        let value = print_and_return_10(5);
        assert_eq!(5, value);
    }
    #[test]
    fn test_add_two1(){
        let value = add_two(7);
        assert_eq!(value, 9);
    }
    #[test]
    fn test_add_two2(){
        let value = add_two(127);
        assert_eq!(value, 129);
    }
    #[test]
    fn test_add_two3(){
        let value = add_two(721);
        assert_eq!(value, 723);
    }
    #[test]
    fn test_add_two4(){
        let value = add_two(700);
        assert_eq!(value,702);
    }
    #[test]
    fn another(){
        let val = add_two(10);
        assert_eq!(val, 12);
    }
    #[test]
    #[ignore] //After #[test] we add the #[ignore] line to the test we want to exclude. Now when we run our tests, it_works runs, but expensive_test doesn’t
    fn expensive_test(){
        for i in 0..99999999 {
            let value = add_two(i);
            assert_eq!(i+2, value);
        }
    }
}
