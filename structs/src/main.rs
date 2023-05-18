#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
impl User {
    fn print(&self) {
        println!("name: {} \nemail: {}\nactive:{}",self.username, self.email, self.active);
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
    fn can_contain(&self, rectangle: &Rectangle) -> bool{
        if self.width > rectangle.width && self.height > rectangle.height {
            return true
        }
        return false
    }
}
fn main() {
    let mut user1: User = User {
        email: String::from("m.aly.ashour@gmail.com"),
        username: String::from("m-ashour"),
        active: true,
        sign_in_count: 1
    };
    println!("{:#?}", user1);
    println!("Hello, world!");
    user1.print();
    let rec1: Rectangle = Rectangle{
        width: 10,
        height: 20
    };
    let rec2: Rectangle = Rectangle{
        width: 5,
        height: 30
    };
    let can_contain_rec2 = rec1.can_contain(&rec2);
    if can_contain_rec2 {
        println!("rec1 can contain rec2");
    }
    else{
        println!("rec1 can't contain rec2");
    }
}
