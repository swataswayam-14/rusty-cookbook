enum Option<T>{
    Some(T),
    None,
}
enum Result <T, E>{ //multiple generic types
    Ok(T), //std::fs::File
    Err(E), //std::io::Error
}