pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn internal_adder(a: i32, b: i32) -> i32 { //Note that the internal_adder function is not marked as pub , tests can be runned on private functions as well
    a + b
}

#[cfg(test)] //cfg stands for configuration and tells Rust that the following item should only be included given a certain configuration option.
//In this case, the configuration option is test, which is provided by Rust for compiling and running tests. By using the cfg attribute, Cargo compiles our test code only if we actively run the tests with cargo test. This includes any helper functions that might be within this module, in addition to the functions annotated with #[test].
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn test_internal_adder(){
        let result = internal_adder(9,9);
        assert_eq!(result, 18);
        let result = internal_adder(91,1);
        assert_eq!(result, 92);
        let result = internal_adder(-9,9);
        assert_eq!(result, 0);
    }
}
