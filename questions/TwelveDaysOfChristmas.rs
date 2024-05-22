// for number in (1..4).rev(){
//     println!("{number}");
// }

fn main(){
    let lyrics: [&str; 12] = ["A partridge in a pear tree", "turtle doves", "French hens", "calling birds","golden rings","geese a-laying", "swans a-swimming", "maids a-milking", "ladies dancing", "lords a-leaping", "pipers piping", "drummers drumming"];

    let numsArray:[&str;12] = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eigth", "nineth", "tenth", "eleventh", "twelveth"];


    for number in 0..12 {
        println!(
            "On the {} day of Christmas,\nmy true love gave to me",
            numsArray[number]
        );

        for count in (0..=number).rev() {
            if count == 0 && number != 0 {
                println!("And {}", lyrics[count]);
            } else {
                println!("{}", lyrics[count]);
            }
        }
    }
}