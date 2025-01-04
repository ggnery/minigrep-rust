pub mod custom_errors;
pub mod config;
mod search;

use std::error::Error;
use config::Config;
use search::{ file_search::FileSearch, Search, SearchType};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config.get_search_type() {
        SearchType::FileSearch => {
            let file_search = FileSearch::build(&config)?;
            let results = file_search.search(file_search.get_query(), &file_search.get_contents() );
            file_search.display(&results);
        },
        SearchType::StringSearch => {todo!("string search not implemented")},
        SearchType::RecursiveSearch => {todo!("recursive search not implemented")}
    }

    Ok(())
}