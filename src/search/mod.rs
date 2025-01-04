pub mod file_search;

pub enum SearchType {
    StringSearch,
    RecursiveSearch,
    FileSearch
}

pub trait Search {
    fn search<'a>(&self, query: &str, contents: &'a str) -> Vec<&'a str>{
        contents
            .lines()
            .filter(|line| line.contains(query))
            .collect()
    }
}