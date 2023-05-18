fn main() {
    println!("Hello, world!");
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number = {number}");

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let mut counter: i32 = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result = {result}");
    // Loop labels
    let mut count = 0;
    // Loop labels
    'counting: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining ={remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("Finished counting!!!");

    //while loop
    while count != 0 {
        println!("count ={count}");
        count -= 1;
    }
    // looping across a collection (Array,...)
    let a = [1, 2, 3, 4, 5, 6, 7];
    for element in a {
        println!("element = {element}");
    }
    // for loop in a range
    for num in (1..5).rev() {
        println!("num = {num}");
    }
}