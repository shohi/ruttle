extern crate regex;
extern crate serde_json;
extern crate stllib;

use regex::{Captures, Regex};
use std::cell::RefCell;
use std::env;
use std::io::prelude::*;
use std::ops::{Deref, DerefMut};

use serde_json::Value;
use stllib::aws::ec2::EC2Proxy;
use stllib::shuttle::config;

fn main() {
    // load config
    let content = config::load_config().unwrap();
    let data: Value = serde_json::from_str(&content).unwrap();
    let val = RefCell::new(data);

    // update config - mainly public ip
    let mut v = val.borrow_mut();
    update_config(v.deref_mut());

    // save config
    let res = val.borrow();
    // println!("content: {:?}", res.to_string());
    let res = config::save_config(res.deref());
    match res {
        Ok(()) => println!("saving config successfully"),
        Err(e) => panic!("error saving config: {:?}", e),
    }
}

//
fn update_config(v: &mut Value) {
    let proxy = EC2Proxy::new();
    let re = Regex::new(r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}").unwrap();

    let hosts: &mut Vec<Value> = v["hosts"].as_array_mut().unwrap();
    for info in hosts.iter_mut() {
        let mut cluster = info.as_object_mut().unwrap();
        for (_key, val) in cluster.iter_mut() {
            let mut group = val.as_array_mut().unwrap();
            for item in group.iter_mut() {
                update_item_public_ip(&proxy, item, &re);
            }
        }
    }
}

//
fn update_item_public_ip(proxy: &EC2Proxy, item: &mut Value, re: &Regex) {
    let typ = item["type"].as_str().unwrap();
    if typ != "AWS" {
        return;
    }

    let inst_id = item["instanceID"].as_str().unwrap();
    let ip = match proxy.get_instance_public_ip(inst_id) {
        Some(s) => s,
        None => return,
    };

    let mut cmd = item["cmd"].as_str().unwrap().to_string();
    cmd = re
        .replace_all(&cmd, |_caps: &Captures| ip.to_owned())
        .to_string();

    // println!("type: {}, inst: {}, ip: {:?}", typ, item, ip);
}
