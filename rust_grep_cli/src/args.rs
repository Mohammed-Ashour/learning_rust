use std::env;

#[derive(Debug,Default)]
pub struct Args {
    pub query: String,
    pub filepath: String
}

impl Args {
    pub fn parse_args() -> Result<Args, String> {
        let args: Vec<String> = env::args().collect();
        if args.len() >= 3 {
            let query = &args[1];
        let filepath = &args[2];
        return Ok(Args{query: String::from(query), filepath:String::from(filepath)});
    } else {
        return Err(String::from("Not Enough Arguments!!"));
    }
    }
}