//default implementation

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
    fn description(&self) {
        println!("Default implementation of description function");
        println!("calling a function that donot have default implementation");

        println!("Called Summarize author: {}", self.summarize_author());
    }
    fn summarize_author(&self) -> String;
}

pub struct Post {
    title: String,
    content: String,
    edited: bool,
    author: String,
    date: String,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("{}: {} by {}", self.title, self.content, self.author)
    }
    fn summarize_author(&self) -> String {
        format!("the author is {}", self.author)
    }
}

pub struct Tweet {
    pub username:String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    // default implementation of the fn summarize applies here
    fn summarize_author(&self) -> String {
        format!("the author is {}", self.username)
    }
}

fn main() {
    let post = Post {
        title:"dskjhfskf".to_string(),
        content:"jgsfksf".to_string(),
        edited:true,
        author:"vksuhfds".to_string(),
        date:"1/2/2024".to_string(),
    };
    println!("{}", post.summarize());
    let tweet = Tweet {
        username: String::from("swataswayam.14"),
        content: String::from("Gotta be the best in open source"),
        reply: true,
        retweet: false,
    };
    println!("The summary of the tweet:    {}", tweet.summarize());
    post.description();
    tweet.description();
}