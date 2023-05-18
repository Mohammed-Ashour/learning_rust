fn main() {
    println!("Hello, world!");
    let mut s: String = String::from("Hello World");
    //Slices are references to part of the string
    //it has pointers to the start of the part of the string and the len
    let hello: &str = &s[..5];
    let world: &str = &s[6..];
    let word = first_word(&s);
    //this will cause an err error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    s.clear();
    println!("{}, is the first word og {}", word, s);
    //but if you added the s.clear() here, it will be ok as the reference scope will be already ended
    
}
//this function will use references to return the first word in a string 
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..]
}
