extern crate toml;
#[macro_use]
extern crate try_opt;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate rusoto_ec2;
extern crate rusoto_core;
extern crate regex;

pub mod shuttle;
pub mod aws;