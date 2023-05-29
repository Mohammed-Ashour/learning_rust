use std::collections::HashMap;
use super::resp::{Response, Req, Status};
#[derive(Debug,Default)]
pub struct Memory{
    group: String,
    map: HashMap<String, String>
}

#[derive(Debug,Default)]
pub struct MemoryMap{
    m:HashMap<String,Memory>
}

impl MemoryMap{
    pub fn set(&mut self, group:&str, key:&str, value:&str) -> Response{
        let mem = self.m.entry(String::from(group)).or_insert(Memory::default());
        let cell = mem.map.entry(key.to_string()).or_insert(String::from(value)).clone();

        let Res = Response{
            Status: Status::Success,
            Msg: cell
        };
        return Res


    }
    pub fn get(&self, group:&str, key:&str)-> Response{
        if !self.m.contains_key(group){
            return Response{
                Status: Status::NotFound,
                Msg: format!("{} Doesn't exist in the memory", group)
            };
        }
        if let Some(memory_cell) = self.m.get(group) {
            if let Some(value) = memory_cell.map.get(key) {
                return Response{
                    Status: Status::Success,
                    Msg: String::from(value)
                };

            }
            else{
                return Response{
                    Status: Status::NotFound,
                    Msg: format!("{} Doesn't exist in the memory", key)
                };

            }
            
        } else {
            return Response{
                Status: Status::NotFound,
                Msg: format!("{} Doesn't exist in the memory", group)
            };
        }


    }
}
// let init_memory = Memory{
//     group: String::from("init"),
//     map: HashMap::new()
// };