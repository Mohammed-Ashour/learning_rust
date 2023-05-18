fn main() {
    println!("Hello, world!");
    let x:i32 = 5;
    let y:i32 =x.clone();


    let s1: String = String::from("hello");
    let mut s2: String =  s1.clone();

    s2  = takes_and_gives_ownership(s2);
    let s2_len = calculate_length(&s2);
    println!("{}, world with length {}", s2, s2_len);

    println!("{}, world", s1);

}
fn gives_ownership() -> String {
    let some_string: String = String::from("hello");
    some_string
}

fn takes_ownership(a_string: String) {
    let a = true;
}

fn takes_and_gives_ownership(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    s.len()
}