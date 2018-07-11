use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Result, Error, ErrorKind};

use serde_json::Value;
use std::fs::OpenOptions;

const SHUTTLE_CONFIG: &str = ".shuttle.json";

// read shuttle default configuration
pub fn load_config() -> Option<String> {
    let path = try_opt!(env::home_dir()).join(SHUTTLE_CONFIG);

    let mut f = match File::open(&path) {
        Err(err) => {
            println!("open path - [{}] - error, err: {}", &path.display(), err);
            return None;
        },
        Ok(f) => f
    };

    let mut contents = String::new();
    match f.read_to_string(&mut contents) {
        Err(err) => {
            println!("something went wrong reading the file, err: {}", err);
            return None;
        }
        Ok(_) => {}
    }
    
    return Some(contents);
}

// read shuttle default configuration
pub fn save_config(v: &Value) -> Result<()>{
    let homedir = env::home_dir();
    let path = match homedir {
        Some(p) => p.join(SHUTTLE_CONFIG), 
        None => return Err(Error::new(ErrorKind::Other, "home dir not found!"))
    };

    let mut file = OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(true)
            .open(path)?;

    file.write_all(Value::to_string(v).as_bytes())?;
    return Ok(());
}