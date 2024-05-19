use std::env;
use std::io::{self, Read};

mod decode;
mod encode;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: jsonbee <encode|decode>");
        std::process::exit(1);
    }

    let operation = &args[1];
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    match operation.as_str() {
        "encode" => match encode::json_to_bencode(&input) {
            Ok(result) => {
                println!("{}", result);
            }
            Err(e) => {
                eprintln!("Failed to parse JSON: {}", e);
                std::process::exit(1);
            }
        },
        "decode" => match decode::bencode_to_json(&input) {
            Ok(json_value) => {
                println!("{}", json_value);
            }
            Err(e) => {
                eprintln!("Failed to decode Bencode: {}", e);
                std::process::exit(1);
            }
        },
        _ => {
            eprintln!("Invalid operation. Use 'encode' or 'decode'.");
            std::process::exit(1);
        }
    }
}
