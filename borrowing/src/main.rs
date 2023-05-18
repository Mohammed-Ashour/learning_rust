fn main() {
    // println!("Hello, world!");
    // let s: String = String::from("A String");
    // // Borrowing the value using the reference 
    // let s_len: usize = get_length(&s);
    // println!("{} with len {}", s, s_len);
    // //You can't have 2 mut references to the same object
    // //ex:
    // let  r1: &mut String = &mut s;
    // let  r2: &mut String = &mut s;
    // println!("{},{}", r1,r2);
    // //error : error[E0499]: cannot borrow `s` as mutable more than once at a time

    // //You can just make them immutable
    // let r1: &String = &s;
    // let r2: &String = &s;
    // println!("{},{}", r1, r2); 
    // println!("{},{}", r1, r2);
    // get_length(&r1);

    let mut s: String = String::from("hello");

    let r1: &String = &s; // no problem
    let r2: &String = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3: &mut String = &mut s; // no problem
    println!("{}", r3);
    println!("{} and {}", r1,  r2);
    // The Rules of references 
    //1: you can have any num of mutable references or only one immutable ref
    //2: Ref scope ends in the last time it has been used
    //3:Ref must always be valid
}

fn get_length(s: &String) -> usize {
    s.len()
}