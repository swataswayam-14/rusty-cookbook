
use std::collections::HashMap;

fn main(){
    //creating a Hash Map
    let mut scores = HashMap::new();

    scores.insert(String::from("BLUE"),10);
    scores.insert(String::from("YELLOW"), 20);

    //accessing values in hash map

    let team_name = String::from("BLUE");
    let score = scores.get(&team_name).copied().unwrap_or(0);//score will have the value that’s associated with the Blue team, and the result will be 10

    //The get method returns an Option<&V>; if there’s no value for that key in the hash map, get will return None. This program handles the Option by calling copied to get an Option<i32> rather than an Option<&i32>, then unwrap_or to set score to zero if scores doesn't have an entry for the key.

    //iterating over a hash map:

    for (key, value) in &scores{
        println!("{key}: {value}");
    }

    //hashmap and ownership

    let field_name = String::from("student name");
    let field_value = String::from("Swayam");

    let mut map = HashMap::new();

    map.insert(field_name, field_value);
    //field_name and field_value are invalid at this point

    //If we insert references to values into the hash map, the values won’t be moved into the hash map. The values that the references point to must be valid for at least as long as the hash map is valid.

    //updating a hashmap
    //1. Overwritting a Value

    let mut score2 = HashMap::new();
    score2.insert(String::from("BLACK"), 10);
    score2.insert(String::from("BLACK"), 20);

    println!("{:?}", score2);


    //2. Adding a key and value only if a key isn't present
    let mut score3 = HashMap::new();
    score3.insert(String::from("BLACK"), 10);

    score3.entry(String::from("YELLOW")).or_insert(30);
    score3.entry(String::from("BLACK")).or_insert(90);
    println!("{:?}", score3);


    //3.Updating a value based on the old value

    //count the frequency of each word in a sentence

    let text = "hello world beautiful world beautiful beautiful hariyali greenery every world has this amazing thing hello hello hello";

    let mut freq = HashMap::new();

    for word in text.split_whitespace(){
        let count = freq.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", freq);
}

//By default, HashMap uses a hashing function called SipHash that can provide resistance to Denial of Service (DoS) attacks involving hash tables