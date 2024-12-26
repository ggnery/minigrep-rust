use std::{env, error::Error};

use minigrep::{config::Config, run};


fn main() -> Result<(),  Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args)?;

    run(config)
}





