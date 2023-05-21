use std::{
    fs::File,
    io,
    io::{ErrorKind, Read},
};
fn main() {
    // println!("Hello, world!");
    // dont_pass_two(2)
    let f = File::open("file.txt");
    //here the Result type has 2 possibilities,
    // Returning a File or an Error
    //here we can match the result
    // let r = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("No file here! {:?}", error)

    // };
    //we can handle it better by creating a file if none exists
    let better_r = match f {
        Ok(fc) => fc,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("file.txt") {
                Ok(f) => f,
                Err(e) => panic!("No Idea {:?}", e),
            },
            other_errors => {
                panic!("I don't know! {:?}", other_errors)
            }
        },
    };
    // this handles the non existant of a file but is extensive
    //you can use unwrap or expect
    let ff = File::open("file2.txt").expect("Error happened");
}
fn read_file() -> Result<String, io::Error> {
    let f = File::open("user.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    }; // this block can be written in smaller format using the ? symbol
       //like this
       // let f = File::open("user.txt")?;
       // the ? symbol will return the file if Ok and the error otherwise

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    } //same here, using the ?
      // f.read_to_string(&mut s)?;
      //Ok(s)
      //and we can chain them like this
      // let f = File::open("user.txt")?.read_to_string(&s)?;
      // and all of that is actually implemented in the fs module
      // fs::read_to_string("user.txt")
}

fn dont_pass_two(num: i32) {
    if num == 2 {
        //panic throws the error and exits the program
        panic!("Told you not to pass 2")
    }
}
