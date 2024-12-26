use std::fs;

use crate::config::Config;
use super::Search;

pub struct FileSearch<'a> {
    query: &'a String,
    file_path: &'a String,
    ignore_case: bool,
}

impl<'a> Search for FileSearch<'a> {}

impl<'a> FileSearch<'a> {
    pub fn new(config: &'a Config) -> FileSearch<'a> {
        FileSearch{
            query: config.get_query(),
            file_path: config.get_file_path(),
            ignore_case: *config.get_ignore_case()
        }
    } 

    pub fn display(&self) {
        let mut contents = fs::read_to_string(self.file_path).unwrap();
        
        let results = if self.ignore_case {
            contents = contents.to_lowercase();
            let query = self.query.to_lowercase();
            
            self.search(&query, &contents)
        }else{
            self.search(self.query, &contents)
        };
        
        for line in results {
            println!("{line}");
        }
    }

}
