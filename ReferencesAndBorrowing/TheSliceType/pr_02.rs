fn main(){
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{0} {1}", hello , world);

    let m = String::from("hello");

    let len = m.len();

    let slice1 = &m[..2];
    let slice2 = &m[0..2]; 
    //slice1 and slice2 are equal

    let slice3 = &m[2..];
    let slice4 = &m[2..len];
    // slice3 and slice4 are equal

    let slice5 = &m[0..len];
    let slice6 = &m[..];

    //slice5 and slice6 are equal


    //slices work by storing a reference to the first element and a length.

    let a = [1, 2, 3, 4, 5];

    let slice_arr = &a[1..3];
}