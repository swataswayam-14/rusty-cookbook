use Test_organization::add;

mod common;

#[test]
fn it_adds_two(){
    assert_eq!(4, add(2,2));
}

#[test]
fn it_adds_two1(){
    assert_eq!(11, add(9,2));
}

#[test]
fn it_checks(){
    common::setup(12, add(9,3), 999);
}
//cargo test --test integration_test  -> This command runs only the tests in the tests/integration_test.rs file.