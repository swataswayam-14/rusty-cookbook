use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String
}

#[derive(Serialize, Deserialize)]
struct Tweet {
    tweet: String,
    author: String,
    paragraph: Vec<Paragraph>
}

fn main() {
    let json = r#"
    {
        "tweet": "a quiter never wins and a winner never quits",
        "author":"Swata Swayam Dash",
        "paragraph": [
            {
                "name": "quitting is not in my genes"
            },
            {
                "name": "I am born to win and achieve everything that is there"
            },
            {
                "name":"I deserve all great things"
            }
        ]
    }
    "#;

    let parsed: Tweet = read_json(json);
    println!("\n\n The name of the first paragraph is: {} and the author of the tweet is {}", parsed.paragraph[0].name, parsed.author);
}

fn read_json(raw_json: &str) -> Tweet {
    let parsed: Tweet = serde_json::from_str(raw_json).unwrap();
    return parsed
}
