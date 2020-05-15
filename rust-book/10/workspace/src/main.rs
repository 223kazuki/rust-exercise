use std::cmp::PartialOrd;
use std::fmt::Display;

pub trait Summary {
    fn summarize(&self) -> String;
}

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
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let number_list = vec![43, 123, 423, 34, 41, 341];
    let largest = largest(&number_list);

    let char_list = vec!['y', 'm', 'a', 'q'];

    println!("Hello, world! {}?", largest);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        // もちろん、ご存知かもしれないようにね、みなさん
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    {
        let r;

        let x = 5;
        r = &x;

        println!("{}", r);
    }

    let c;
    let a = String::from("AAA");
    {
        let b = "BBB";
        c = longest(a.as_str(), b);
    }
    println!("{}", c);

    let c;
    {
        let a = String::from("AAA");
        let b = "BBB";
        c = longest2(a.as_str(), b);
    }
    println!("{}", c);

    let i;
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("{:?}", i.part);

    let s = String::from("A");
    let a = longest_with_an_announcement("AAAA", "AAAAAAA", &s);
    println!("{}", a);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest2(x: &str, y: &str) -> String {
    if x.len() > y.len() {
        String::from(x)
    } else {
        String::from(y)
    }
}
