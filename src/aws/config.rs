use toml;
use std::io::prelude::*;

#[derive(Deserialize, Debug)]
pub struct AwsConfig {
    region: String,
    key_id: String,
    access_key: String
}

// 
pub fn load_config() -> Result<AwsConfig, String> {
    let config_str = include_str!("config.toml");
    let value = config_str.parse::<toml::Value>().unwrap();
    let aws_config = value["aws"].clone().try_into::<AwsConfig>().unwrap();

    Ok(aws_config)
}
