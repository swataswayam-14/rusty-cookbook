//Lifetime Elision

fn first_word (s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
} //The reason this function compiles without lifetime annotations is historical: in early versions (pre-1.0) of Rust, this code wouldn’t have compiled because every reference needed an explicit lifetime.

fn first_word<'a>(s: &'a str) -> &'a str {
    
}