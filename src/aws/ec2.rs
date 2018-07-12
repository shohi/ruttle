use super::config;

use std::default::Default;

use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_ec2::{Ec2Client, Ec2, DescribeInstancesRequest, Reservation};


pub struct EC2Proxy {
    client: Ec2Client
}

impl EC2Proxy {
    pub fn new() -> EC2Proxy {
        return EC2Proxy{
            client: Ec2Client::simple(Region::ApSoutheast1)
        }
    }

    pub fn get_instance_public_ip(&self, instID: String) -> Option<String> {
        let insts: Vec<String> = vec![instID];

        let req = DescribeInstancesRequest {
            instance_ids: Some(insts),
            ..Default::default()
        };
        
        match self.client.describe_instances(&req).sync() {
            Ok(output) => {
                match output.reservations {
                    Some(rsvs) => {
                        if rsvs.len() == 0 {
                            return None
                        }
                        return self.extract_first_public_ip(&rsvs[0])
                    },
                    None => println!("No tables in database!"),
                }
            },
            Err(error) => {
                println!("Error: {:?}", error);
            },
        }

        return None;
    }

    fn extract_first_public_ip(&self, rsv: &Reservation) -> Option<String> {
        match &rsv.instances {
            Some(insts) => {
                if insts.len() == 0 {
                    return None
                }
                match insts.first() {
                    Some(ref v) => {
                        return (&v.public_ip_address).clone().to_owned()
                    },
                    None => {
                        return None
                    },
                }
            },
            None => {
                return None
            },
        };
    }
}


#[cfg(test)]
#[path = "./ec2_test.rs"]
mod ec2_test;
