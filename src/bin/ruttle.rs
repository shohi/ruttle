extern crate serde_json;
extern crate stllib;

use std::env;
use std::io::prelude::*;

use serde_json::{Value};
use stllib::shuttle::config;

fn main() {
    // load config
    let content = config::load_config().unwrap();
    let v:Value = serde_json::from_str(&content).unwrap();
    
    // update aws public ip
    let hosts: &Vec<Value> = v["hosts"].as_array().unwrap();
    for info in hosts.iter() {
        println!("{}", Value::to_string(&info));
        let mut cluster = info.as_object().unwrap();
        for (ref key, ref val) in cluster.iter() {
            // println!("{}, {}", key, val);
            println!("{}", key);
            let group = val.as_array().unwrap();
            for inst in group.iter() {
                println!("inst: {}", inst)
            }
        }
    }
    // save config
    let res = config::save_config(&v);
    match res {
        Ok(()) => println!("saving config successfully"),
        Err(e) => println!("error saving config: {:?}", e),
    }
}






