//Testing Equality with the assert_eq! and assert_ne! Macros

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub fn add_two(a: i32) -> i32 {
    a + 2
}
pub fn contains_string(a: &str) -> String {
    format!("hello {}",a)
}
pub fn contains_string2(a: &str) -> String {
    String::from("Professor")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn test_add_two(){
        assert_eq!(4, add_two(2));
    }
    #[test]
    fn test_add_two2(){
        assert_ne!(-1, add_two(2));
    }
    #[test]
    fn test_add_two3(){
        assert_ne!(-1, add_two(-3));
    }

    #[test]
    fn test_contains(){
        let result = contains_string("tokyo");
        assert!(result.contains("tokyo"), "the function did not contained tokyo, it has the value: {}",result);
    }
    #[test]
    fn test_contains2(){
        let result = contains_string2("tokyo");
        assert!(result.contains("tokyo"), "the function did not contained tokyo, it has the value: {}",result);
    }

}
