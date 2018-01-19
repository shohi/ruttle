// use std::path::Path;
use std::fs;

const SHUTTLE_CONFIG: &str = "～/.shuttle.json";

fn main() {
    let path = fs::canonicalize("～/.shuttle.json")?;
    println!("hello, world! {}", SHUTTLE_CONFIG);
    println!("path ==> {}", path.display());
}
