use std::{error::Error, fs};

use crate::config::Config;
use super::Search;

pub struct FileSearch {
    query: String,
    file_path: String,
    contents: String,
    ignore_case: bool,
}

impl Search for FileSearch {}

impl FileSearch {
    pub fn build(config: &Config) -> Result<FileSearch, Box<dyn Error>> {
        let mut contents = fs::read_to_string(config.get_file_path()).unwrap();
        let mut query = config.get_query().clone();
        let file_path =  config.get_file_path().clone();
        let ignore_case = config.get_ignore_case().clone();

        if ignore_case {
            contents = contents.to_lowercase();
            query = query.to_lowercase();
        }

        Ok(FileSearch{
            query,
            file_path,
            ignore_case,
            contents
        })
    } 

    pub fn display(&self, results: &Vec<&str>) {
        for line in results {
            println!("{line}");
        }
    }

    pub fn get_contents(&self) -> &String {
        &self.contents
    }

    pub fn get_query(&self) -> &String {
        &self.query
    }
}
