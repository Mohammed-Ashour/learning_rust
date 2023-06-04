use std::fs;
use std::error::Error;

#[derive(Debug,Default, Clone)]
pub struct File{
    pub path: String,
    pub content: String
}
impl File{
    pub fn read_file(&mut self, filepath:String) -> Result<(), Box<dyn Error>>{
        self.content = fs::read_to_string(filepath)?;
        println!("{}", self.content);
        return Ok(())
    
    }
    pub fn search(&self, query: &str) -> Vec<&str> {
        let mut matched_lines = Vec::new();
        for line in self.content.lines(){
            if line.contains(query) {
                matched_lines.push(line);
            }
        }
        return matched_lines;

    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_search(){
        let file: File = File{
            path: String::from("fake_path"),
            content: String::from("Hello \nname1\nname2\nnames")
        };
        assert_eq!(vec!["name1", "name2", "names"], file.search("name"));

    }


}