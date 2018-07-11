use super::config;

use std::default::Default;

use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_ec2::{Ec2Client, Ec2};


pub fn create_client() -> Ec2Client {
    Ec2Client::simple(Region::ApSoutheast1)
}

fn single_number() -> i32 {
    return 32;
}


#[cfg(test)]
#[path = "./ec2_test.rs"]
mod ec2_test;
