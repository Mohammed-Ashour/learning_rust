use std::fmt::Error;


#[derive(Debug)]
    pub struct Req {
        pub cmd: Command,
        pub typ: ReqType,
        pub group: String,
        pub name: String,
        pub value: String,

    }
    impl Req{
        pub fn new( request_body:String) -> Req{

            let mut clean_tokens: Vec<String> = Vec::new();
            let tokens = request_body.split("\r\n");
            for token in tokens {
                if let Some(first_char) = token.chars().next() {
                    if first_char != '\0' {
                        clean_tokens.push(String::from(token))
                    }
                }
            }
            let request_type = ReqType::new(&clean_tokens[1]);
            let cmd = Command::new(&clean_tokens[2]);
            let name = String::from(&clean_tokens[4]);
            let value = String::from(&clean_tokens[6]);
            let req = match cmd {
                Command::Set =>Req {
                    cmd: cmd,
                    group: name,
                    typ: request_type,
                    name: value,
                    value: String::from(&clean_tokens[8])
                } ,
                _ => Req {
                    cmd: cmd,
                    group: name,
                    typ: request_type,
                    name: value,
                    value: String::from("")
                } 
            };
            
            return req
            

        }
        pub fn get_name(&self) -> &str{
            return self.name.as_str()
        }
        pub fn get_value(&self) -> &str{
            return self.value.as_str();
        }
        pub fn get_group(&self) -> &str{
            return self.group.as_str();
        } 
    }
#[derive(Debug)]
    pub enum ReqType{
        String,
        Num,
        Other,
    }
    impl ReqType{
        fn new(type_str:&str) -> ReqType{
            match &type_str[..1]{
                "$" => ReqType::String,
                ":" => ReqType::Num,
                _ => ReqType::Other
            }
        }
    }

#[derive(Debug)]
pub enum Command{
    Ping,
    Set,
    Get,
    Unknown
}

impl Command{
    pub fn new(cmd:&str) -> Command {
        match cmd.to_lowercase().as_str() {
            "set" => Command::Set,
            "get" => Command::Get,
            "ping" =>Command::Ping,
            _ => Command::Unknown
            
        }

    }
}

#[derive(Debug)]
pub enum Status{
     Success,
     NotFound,
     Error
}

#[derive(Debug)]
pub struct Response{
    pub Status: Status,
    pub Msg: String
}
