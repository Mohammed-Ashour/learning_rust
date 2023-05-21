/// Here we will talk about generics, traits and lifetimes
///
use std::fmt::Display;
/// <'a> here is a lifetime
/// when you return a ref to an input, the borrow checker will need to know
/// the lifetime of the result/output. and that will depend on the smallest lifetime
/// of the inputs
/// the annotation is just to unify the lifetimes, not to create a new one
fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display, //here is a trait to tell the compiler that generic T will be
                //any type that implements Display (can be printed)
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        return x;
    } else {
        return y;
    }

}


///here we use both generics (T) which makes the function able to deal with
/// different types without needing to create separate function for each one
/// and we are using traits to say that the generic types that we are accepting here
/// must have implementation to both PartialOrd (comparison) and Copy traits
/// which will enable the comparison and returning the value inside the function
/// 
fn get_largest<T: PartialOrd+Copy>(number_list:Vec<T>) -> T {
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    return largest

}
/// generics inside structs
/// here, this struct can have any type and also both x and y can be of different types
 
struct Point<T,U>{
    x:T,
    y:U,
}
// Here you can see that the naming of the generics doesn't have to be the same
// as in the struct def
impl <A,B> Point<A,B> {
    fn x(&self)->&A{
        return &self.x
    }
    fn y(&self)->&B{
        return &self.y
    }
}
//traits are defined like interfaces, you don't need to add an implementation 
//to your functions here, also you can add a default implementation

pub trait Summary {
    fn summarize (&self) -> String;
}
pub struct Tweet {
    pub username: String, 
    pub content: String,
    pub reply: bool, 
    pub retweet:bool
}
impl Summary for Tweet {
    fn summarize (&self) -> String {
        return format!("{}: {}",self.username, self.content);
    }
}


fn main() {

}
