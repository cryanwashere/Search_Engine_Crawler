

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


//use futures::executor::block_on;
mod crawl;
use std::fs;
use tokio;

#[tokio::main]
async fn main() {
    // start crawling...
    crawl::initialize_crawl().await;

   


}



// Ignore this

async fn upsert_test() {
    let upsert_res = crawl::send_image_url("https://upload.wikimedia.org/wikipedia/commons/thumb/7/73/Monarch_Butterfly_Danaus_plexippus_Male_2664px.jpg/220px-Monarch_Butterfly_Danaus_plexippus_Male_2664px.jpg","https://en.wikipedia.org/wiki/Monarch_butterfly");

    match upsert_res.await {
        Ok(message) => {
            println!("{}", message);
        }
        Err(err) => {
            println!("Error fetching html content: {}", err);
        }
    }
}

fn parse_html_test() {

    let html_content = fs::read_to_string("monarch_butterfly.html").expect("Unable to read file");
    

    let parse_result = crawl::parse::extract_wikipedia_HTML(&html_content, "https://en.wikipedia.org/wiki/Monarch_butterfly");

    parse_result.print_info();

    parse_result.make_content_html("monarch_butterfly_content.html");
}

