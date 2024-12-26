use std::{env, path::Path};

use crate::{custom_errors::GreepError, search::SearchType};

pub struct Config<'a> {
    query: Option<&'a String>,
    file_path: Option<&'a String>,
    source_path: Option<&'a String>,
    string_buff: Option<&'a String>,
    search_type: Option<SearchType>,
    ignore_case: bool,

}



impl<'a> Config<'a> {
    pub fn build(args: &Vec<String>) -> Result<Config, GreepError> {
 
        let file_path = None;
        let source_path= None;
        let string_buff = None;
        let search_type = None;
        let query = None;
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        let default_config = Config {
            file_path,
            source_path,
            string_buff,
            ignore_case,
            query,
            search_type
        };

        match args.len() {
            n if n < 3 => { return Err(GreepError::NotEnoughtArgs); },
            n if n > 4 => { return  Err(GreepError::ManyArgs); },
            n if n == 4 => { 
                if args[1] != "-r" {
                    return Err(GreepError::GenericError("Invalid command"));
                }else{
                    return Ok(Config {
                        query: Some(&args[2]),
                        source_path: Some(&args[3]),       
                        search_type: Some(SearchType::RecursiveSearch),
                        ..default_config
                    });
                }
            },
            n if n == 3 => {
                if args[1] == "-r" { return Err(GreepError::GenericError("Invalid command")); } 

                if Path::new(&args[2]).is_file() {
                    return Ok(Config {
                        query: Some(&args[1]),
                        file_path: Some(&args[2]),
                        search_type: Some(SearchType::FileSearch),
                        ..default_config
                    })
                }else{
                    return Ok(Config {
                        query: Some(&args[1]),
                        file_path: Some(&args[2]),
                        string_buff: Some(&args[2]),
                        search_type: Some(SearchType::StringSearch),
                        ..default_config
                    })
                }
            },
            _ => { return Err(GreepError::GenericError("Unexpected error"));}
        }
        
    }

    pub fn get_search_type(&self) -> &SearchType {
       self.search_type.as_ref().expect("search_type field not present")
    }

    pub fn get_file_path(&self) -> &String {
        &self.file_path.expect("file_path field not present")
    }

    pub fn get_query(&self) -> &String {
        &self.query.expect("query field not present")
    }
    
    pub fn get_ignore_case(&self) -> &bool {
        &self.ignore_case
    }

}