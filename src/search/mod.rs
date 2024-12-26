pub mod file_search;

pub enum SearchType {
    StringSearch,
    RecursiveSearch,
    FileSearch
}

pub trait Search {
    fn search<'a>(&self, query: &str, contents: &'a str) -> Vec<&'a str>{
        let mut result: Vec<&str> = Vec::new();
    
        for line in contents.lines() {
            if line.contains(query) {
                result.push(line);
            }
        }
    
        result
    }
}