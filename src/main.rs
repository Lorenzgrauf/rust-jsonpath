use std::env;

use json_path::JsonPath;
mod json_path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: json_path <json_path>");
        return;
    }

    let json_path = JsonPath {
        body: args[1].clone()
    };
    
    println!("{:?}", json_path)
}
