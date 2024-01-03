

//use bincode;
//use std::fs;
/*
    let target : Option<String> = Some("Hello".to_string());

    let encoded : Vec<u8> = bincode::serialize(&target).unwrap();

    fs::write("binary_string.bin", encoded).expect("Unable to write binary structure to disk");

    let read_encoded = fs::read("binary_string.bin").expect("Unable to read binary file from disk");

    let decoded : Option<String> = bincode::deserialize(&read_encoded[..]).unwrap();

    println!("decoded string: {:?}", decoded);
*/

mod crawl;
use futures::executor::block_on;
use tokio;

#[tokio::main]
async fn main() {
    crawl::initialize_crawl().await;

}