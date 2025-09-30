fn main() {

    lifetime();

    trait_summary();

    generics_point();

    find_largest();

    trait_boundary();

}


fn lifetime() {
    let string1 = String::from("abcd");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


fn trait_summary() {

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

}



pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {} ...)", self.summarize_author())
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify3<T>(item: &T) where T: Summary {
    println!("Breaking news! {}", item.summarize());
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
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
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    //fn summarize(&self) -> String {
    //    format!("{}: {}", self.username, self.content)
    //}
}




fn generics_point() {

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };


    println!("{:?}, x = {}", integer, integer.x());
    println!("{:?}, x = {}, d = {}", float, float.x(), float.distance_from_origin());
    println!("{:?}, x = {}", integer_and_float, integer_and_float.x());


    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("{:?}", p3);
}

#[derive(Debug)]
struct Point<T,U> {
    x: T,
    y: U
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<V, W>(self, other:Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


fn find_largest() {

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest2(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest2(&char_list);
    println!("The largest char is {}", result);

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


fn largest2<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest : &T = &(list[0]);

    for item in list.iter() {
        if item > largest {
            largest = &item;
        }
    }
    largest
}

use std::cmp::Ordering;
use std::fmt::{Display, Error, Formatter};
use std::result::Result;

fn trait_boundary() {

    let p1 = Pair {x: 3, y: 6};
    let p2 = Pair {x: Aaa {a: 5}, y: Aaa {a:9}};

    p1.cmp_display();
    p2.cmp_display();
}

struct Aaa {
    a: i32
}

impl Display for Aaa {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        Result::Ok(())
    }
}
impl PartialEq for Aaa {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}
impl PartialOrd for Aaa {
    fn partial_cmp(&self, other: &Aaa) -> Option<Ordering> {
        Option::None
    }
}



struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
