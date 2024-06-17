use std::env;
use std::process;

use minigrep::Config;
//We add a use minigrep::Config line to bring the Config type from the library crate into the binary crate’s scope, and we prefix the run function with our crate name. 

fn main(){
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    }); //If the Result is an Ok value, this method’s behavior is similar to unwrap: it returns the inner value Ok is wrapping. However, if the value is an Err value, this method calls the code in the closure, which is an anonymous function we define and pass as an argument to unwrap_or_else.

    if let Err(e) = minigrep::run(config) { //here we have used if let rather than unwrap_or_else to check whether run returns an Err value and call process::exit(1)
        eprintln!("Application error: {e}");
        process::exit(1);
    }
    // The run function doesn’t return a value that we want to unwrap in the same way that Config::build returns the Config instance. Because run returns () in the success case, we only care about detecting an error, so we don’t need unwrap_or_else to return the unwrapped value, which would only be ().
}