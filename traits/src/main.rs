use std::{fmt::format, thread::sleep, iter::Sum};

pub struct NewsArtical {
    pub author: String,
    pub headline: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub retweet: bool,
    pub reply: bool,
}

// trait allows us to define methods that are supported by different types
pub trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for NewsArtical {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.username, self.content)
    }
}

// trait bound example with impl syntax
pub fn notify(item: &impl Summary) {
    println!("Breaking News....{}", item.summarize());
}

// returning any type that implement summary 
fn returns_summarizable () -> impl Summary {
    Tweet {
        username: String::from("@imdhruv99"),
        content: String::from("Hello, from Rust trait"),
        reply: false,
        retweet: false
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("@imdhruv99"),
        content: String::from("Hello, from Rust trait"),
        reply: false,
        retweet: false
    };

    let news_artical = NewsArtical {
        author: String::from("Dhruv Prajapati"),
        headline: String::from("Rust Learning"),
        content: String::from("Hello, from Rust trait")
    };

    println!("Tweet Summary: {}", tweet.summarize());
    println!("News Artical Summary: {}", news_artical.summarize());

    notify(&news_artical);

    println!("{}", returns_summarizable().summarize());
}