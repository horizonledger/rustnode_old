use simple_logger::SimpleLogger;

extern crate serde;
extern crate serde_json;
use std::fs;

use serde::{Deserialize, Serialize};

fn read_file(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}


fn main()  -> Result<(), std::io::Error> {
    SimpleLogger::new().init().unwrap();

    log::info!("example");

    let contents = fs::read_to_string("example.json")?;
    println!("File contents: {}", contents);
    Ok(())

    
}