pub fn setup (x: usize, y: usize, i: i32) {
    if  i == 0 {
        return;
    }
    assert_eq!(x,y);
    setup(x,y,i-1);
}