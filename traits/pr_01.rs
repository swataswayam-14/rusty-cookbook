//defining a trait

pub trait Summary {
    fn summarize(&self) -> String;
}

//implementing a Trait on a Type

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username:String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let news_artice = NewsArticle {
        headline: String::from("Breaking news"),
        location: String::from("Bhubaneswar"),
        author:String::from("Swata Swayam Dash"),
        content:String::from("There is a massive outburst in cuttack"),
    };
    println!("The summary of the news:    {}", news_artice.summarize());

    let tweet = Tweet {
        username: String::from("swataswayam.14"),
        content: String::from("Gotta be the best in open source"),
        reply: true,
        retweet: false,
    };
    println!("The summary of the tweet:    {}", tweet.summarize());
}