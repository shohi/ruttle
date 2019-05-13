mod tests {
    use super::super::*;
    use regex::Regex;

    #[test]
    fn get_inst_public_ip() {
        let proxy = EC2Proxy::new();
        let ip = proxy.get_instance_public_ip("i-07e6a83e3f9a3e2aa");
        println!("get ip: {:?}", ip);
    }

    #[test]
    fn match_ip() {
        let re = Regex::new(r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}").unwrap();
        let is_match = re.is_match("127.0.0.1");
        println!("is match: {}", is_match);
    }
}
