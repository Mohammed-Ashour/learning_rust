
use unicode_segmentation::UnicodeSegmentation;
use std::collections::HashMap;
fn main() {
    println!("Hello, world!");
    let mut v1:Vec<i32> = Vec::new();
    v1.push(1);
    let mut v2:Vec<i32> = vec![1,2,3];
    //accessing elements
    
    let third: &i32 = &v2[2];
    println!("The third element = {}", third);
    
    //accessing with handling invalid index
    //as vectors don't have fixed size, they are stored in the heap
    //and with that, if the index is invalid/out_of_bounds, you will get a runtime error, not compile-time
    match v2.get(4) {
        Some(third) => println!("{}",third),
        None => println!("Nooooo!")
        
    }

    //here you can't have this mutable refrence as you have an immutable ref (third) above
    v2.push(5);
    //as this line makes the `third` var back into scope
    // println!("{}",third);

    ///Loop across the vector
    /// 
    for i in &mut v2{
        // the derefrence operator to get the value and change it
        *i += 50;
    }
    println!("{:?}", v2);
    //we can use this enum to make our vector store different types
    enum DifferentTypes {
        Int(i32),
        Float(f64),
        Text(String)   
    }
    let mut container = vec![
        DifferentTypes::Int(3),
        DifferentTypes::Text(String::from("Hello")),
        DifferentTypes::Float(1.234)
    ];
    match &container[1] {
        DifferentTypes::Int(i) => println!("{}", i),
        _ => println!("Not Int")
    }
    // Strings
    //In rust, strings are in utf-8 encoded bytes
    //utf-8 is a variant length encoding (each char can have a different size than the other)
    //creating string
    let s1 = String::new();
    let s2 = "string";
    let s3 = s2.to_string();
    let s4 = String::from("مرحبا");
    let s5 = s3 + &s4;
    println!("{}",s5);
    //in rust, Strings are represnted in different ways
    //Bytes
    println!("Bytes");
    for b in s5.bytes(){
        println!("{}", b);
    }
    //Scalar values
    println!("Scalar");
    for c in s5.chars(){
        println!("{}", c);
    }
    //Grapheme clusters (normal characters)
    //here we need an external package to use
    println!("Graphemes");
    for g in s5.graphemes(true){
        println!("{}",g);
    }

    let g_s5 = s5.graphemes(true).collect::<Vec<&str>>()[6];
    println!("{}", g_s5);


    ////////Hashmaps
    
    let blue = String::from("blue");
    let yellow = String::from("yellow");
    let mut scores = HashMap::new();
    scores.insert(blue, 10);
    scores.insert(yellow, 33);
    scores.entry(String::from("yellow")).or_insert(22);
    scores.entry(String::from("green")).or_insert(12);

    // creating a count hashmap

    let text = "hello world hello again";
    let mut count_map = HashMap::new();
    for word in text.split_whitespace() {
        let count = count_map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", count_map);

    


}
