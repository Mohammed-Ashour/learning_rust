use std::{process};
mod args;
mod file_op;
use args::Args;
use file_op::File;
fn main() {
    let args= Args::parse_args().unwrap_or_else(|err|{
        println!("Problem with parsing arguments : {}", err);
        process::exit(1);
    });
    let mut f = File::default();
    
    if let Err(e) = f.read_file(args.filepath){
        println!("Error during reading the file");
        process::exit(1)
    };
    println!("{}", f.content);
    println!("Searching for {}", &args.query);
    let results = f.search(&args.query);
    println!("Found {} here {:?}",&args.query, results);


}

