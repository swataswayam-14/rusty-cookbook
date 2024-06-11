use aggregator::{Summary ,Tweet};

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize()); //Instead of a concrete type for the item parameter, we specify the impl keyword and the trait name. This parameter accepts any type that implements the specified trait. In the body of notify, we can call any methods on item that come from the Summary trait, such as summarize. We can call notify and pass in any instance of NewsArticle or Tweet. 
}

//below syntax is more verbose
pub fn notify<T:Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify(item1: &impl Summary , item2: &impl Summary) {

}

//below syntax is more verbose
pub fn notify<T: Summary> (item1: &T, item2: &T){
    //The generic type T specified as the type of the item1 and item2 parameters constrains the function such that the concrete type of the value passed as an argument for item1 and item2 must be the same.
}



//Specifying Multiple Trait Bounds with the + Syntax
pub fn notify(item &(impl Summary + Tweet)){

}

pub fn notify<T:Summary + Tweet> (item: &T){

}

//Clearer Trait Bounds with where Clauses
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {

}
//instead we can write like this: 

fn some_function<T, U> (t: &T, u: &U) -> i32 
where
    T:Summary + Tweet,
    U: Clone + Debug,
{

} 

//Returning Types that Implement Traits

fn returns_summarizable() -> impl Summary {
    Tweet{
        username : String::from("shfiusf"),
        content: String::from(
            "ksjhfsgfjsgf ksdjfjds dsjfsd",
        ),
        reply: false,
        retweet: false,
    }
}
fn returns_summarizable(switch: bool) -> impl Summary { //Returning either a NewsArticle or a Tweet isn’t allowed due to restrictions around how the impl Trait syntax is implemented in the compiler. 
    if switch {
        NewsArticle {
            headline: String::from("kjshdfkhf"),
            location: String::from("ksjdfudgsf"),
            author: String::from("kjhfsdkf")
        }
    }else {
        Tweet {
            username:String::from("sfdkjgsdjbfs"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
}


