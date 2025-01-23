fn main() {
    let color = String::from("green");
    let print = move || println!("`color`: {}", color);
    print();
    print();

    let _reborrow = &color;
    println!("{}", color);
}//TODO: make it work with least amount of changes