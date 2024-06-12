//Lifetime Annotations in Struct Definitions

struct ImportantExcerpt <'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("hey you. Read more....");
    let first_sentence = novel.split('.').next().expect("could not find a '.' ");
    let i  = ImportantExcerpt {
        part: first_sentence,
    };
}