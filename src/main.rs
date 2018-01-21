extern crate serde_json;

#[macro_use]
extern crate try_opt;

use std::env;
use std::fs::File;
use std::io::prelude::*;

use serde_json::{Value};

const SHUTTLE_CONFIG: &str = ".shuttle.json";

fn main() {
    let content = read_shuttle_config().unwrap();

    let v: Value = serde_json::from_str(&content).unwrap();
    let hosts: &Vec<Value> = v["hosts"].as_array().unwrap();
    for info in hosts.iter() {
        let mut cluster = info.as_object().unwrap();
    }
}

// read shuttle default configuration
fn read_shuttle_config() -> Option<String> {
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
            println!("sth went wrong reading the file, err: {}", err);
            return None;
        }
        Ok(_) => {}
    }
    
    return Some(contents);
}


// 
