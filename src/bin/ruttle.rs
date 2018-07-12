extern crate serde_json;
extern crate stllib;

use std::env;
use std::io::prelude::*;

use serde_json::{Value};
use stllib::shuttle::config;
use stllib::aws::ec2::EC2Proxy;

fn main() {
    
    // load config
    let content = config::load_config().unwrap();
    let v: Value = serde_json::from_str(&content).unwrap();
    
    // update aws public ip
    let proxy = EC2Proxy::new();

    let hosts: &Vec<Value> = v["hosts"].as_array().unwrap();
    for info in hosts.iter() {
        let mut cluster = info.as_object().unwrap();
        for (key, val) in cluster.iter_mut() {
            let mut group = val.as_array().unwrap();
            for ref inst in group.iter() {
                let typ = inst["type"].as_str().unwrap();
                if typ == "AWS" {
                    let instID = inst["instanceID"].as_str().unwrap();
                    // println!("type: {}, inst: {}", typ, inst)
                    let ip = match proxy.get_instance_public_ip(instID.to_string()) {
                        Some(s) => s, 
                        None => continue,
                    };
                    println!("type: {}, inst: {}, ip: {:?}", typ, inst,ip)
                }
            }
        }
    }
    // save config
    /*
    let res = config::save_config(&v);
    match res {
        Ok(()) => println!("saving config successfully"),
        Err(e) => println!("error saving config: {:?}", e),
    }
    */
}
