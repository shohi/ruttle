
mod tests {
    use super::super::*;

    #[test]
    fn get_inst_public_ip() {
        let proxy = EC2Proxy::new();
        let ip = proxy.get_instance_public_ip("i-07e6a83e3f9a3e2aa".to_string());
        println!("get ip: {:?}", ip);
    }
}

