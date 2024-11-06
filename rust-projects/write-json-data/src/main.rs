use::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Content {
    name: String
}

#[derive(Serialize, Deserialize)]
struct Post {
    title: String,
    author: String,
    content: Vec<Content>
}

fn main() {
    let post: Post = Post {
        title: String::from("Launch of 10 Apps in playstore"),
        author: String::from("Swata Swayam Dash"),
        content: vec![
            Content {
                name: String::from("5 gaming apps, 3 utility apps, 2 personal apps")
            },
            Content {
                name: String::from("Ad revenue expected to reach at 8.7cr arr")
            }
        ]
    };
    let json = serde_json::to_string(&post).unwrap();
    println!("The json is: {}", json);
}
