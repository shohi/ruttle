// use std::path::Path;
use std::fs;

const SHUTTLE_CONFIG: &str = "~/.shuttle.json";

fn main() {
    let path = fs::canonicalize("~");
    match path {
        Ok(buffer) => {
            buffer.display();
            println!("hello world");
        },
        Err(err) => {
            println!("error occurs, {:?}", err);
            panic!("exit");
        }
    };
    println!("hello, world! {}", SHUTTLE_CONFIG);
    println!("path ==> {}", "hello");
}
