pub mod custom_errors;
pub mod config;
mod search;

use std::error::Error;
use config::Config;
use search::{ file_search::FileSearch, SearchType};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config.get_search_type() {
        SearchType::FileSearch => {
            let file_search = FileSearch::new(&config);
            file_search.display();
        },
        SearchType::StringSearch => {todo!()},
        SearchType::RecursiveSearch => {todo!()}
    }

    Ok(())
}