use super::config;

use std::default::Default;

use rusoto_core::{HttpClient, Region};
use rusoto_credential::ProfileProvider;
use rusoto_ec2::{DescribeInstancesRequest, Ec2, Ec2Client, Reservation};

pub struct EC2Proxy {
    client: Ec2Client,
}

impl EC2Proxy {
    pub fn new() -> EC2Proxy {
        let provider = ProfileProvider::new().unwrap();
        let http_client = HttpClient::new().unwrap();
        let client = Ec2Client::new_with(http_client, provider, Region::ApSoutheast1);

        EC2Proxy { client: client }
    }

    pub fn get_instance_public_ip(&self, inst_id: &str) -> Option<String> {
        let insts: Vec<String> = vec![inst_id.to_string()];

        let req = DescribeInstancesRequest {
            instance_ids: Some(insts),
            ..Default::default()
        };

        match self.client.describe_instances(req).sync() {
            Ok(output) => match output.reservations {
                Some(rsvs) => {
                    if rsvs.len() == 0 {
                        return None;
                    }
                    return self.extract_first_public_ip(&rsvs[0]);
                }
                None => println!("No tables in database!"),
            },
            Err(error) => {
                println!("inst: {}, error: {:?}", inst_id, error);
            }
        }

        return None;
    }

    fn extract_first_public_ip(&self, rsv: &Reservation) -> Option<String> {
        match &rsv.instances {
            Some(insts) => {
                if insts.len() == 0 {
                    return None;
                }
                match insts.first() {
                    Some(ref v) => return (&v.public_ip_address).clone().to_owned(),
                    None => return None,
                }
            }
            None => return None,
        };
    }
}

#[cfg(test)]
#[path = "./ec2_test.rs"]
mod ec2_test;
