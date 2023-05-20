#[derive(Debug)]
enum IPVersion {
    V4,
    V6
}
#[derive(Debug)]
struct IpAddr {
    kind: IPVersion,
    address: String
}
//enums can be in different types
// #[derive(Debug)] is to add the capability for the type to be printed
#[derive(Debug)]
enum Message{
    Quit,
    Move {x:i32, y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}
impl Message {
    fn some_function(&self) {
        println!("{:#?}", self);
    }
}
//option enum

// enum SomeOption<T> {
//     Some(T),
//     None
// }


///
/// Pattern Matching
/// 

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Dime => 10,
        //here is an example of multiline matching block
        Coin::Quarter => {
            println!("Yey!");
            25
        }
    }
}

// Using the Match expr with optional enum

fn plus_one(x: Option<i32>) -> Option<132>{
    match x {
        None => None,
        Some(i) => Some(i+1),
        
    }
    //can also be written like this 
    // match x {
    //     Some(i) => Some(i+1),
    //     _ => None, //this is equivalent to `else`
        
    // }
}
fn main() {
    let ip1: IpAddr = IpAddr{
        kind: IPVersion::V4,
        address: String::from("127.0.0.1")
    };
    let message : Message = Message::Quit;
    message.some_function();

    println!("{:#?}", ip1);
    //option enum used as an option type that can be none 
    let some_number = Some(5);
    let some_string = Some("String...");
    let absent_value: Option<i32> = None;
    //to use the option type for operations we will need to extract the values from it first

    //example
    let sum: i32 = 5 + some_number.unwrap_or(0);



}
