pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub fn mul(left: usize, right: usize) -> usize {
    left * right
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
