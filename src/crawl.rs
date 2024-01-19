use async_recursion::async_recursion;
use reqwest;

use std::collections::HashSet;

pub mod parse;





async fn fetch_html_content(url: &str) -> Result<String, reqwest::Error> {
    /*

        Make an HTTP request to get the HTML content for the given URL
    
    */
    let response = reqwest::get(url).await?;
    let html_content = response.text().await?;
    Ok(html_content)
}

pub async fn send_image_url(image_url: &str, page_url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();

    let body_string = format!("{{\"image_url\": \"{}\", \"page_url\":\"{}\"}}", image_url, page_url);

    //println!("body string: {}",body_string);
   
    let res = client.post("http://139.59.167.5:80/upsert_image_url").header("Content-Type", "application/json").body(body_string).send().await?;

    let message = res.text().await?;

    Ok(message)
}





#[async_recursion]
async fn recursive_page_crawl(crawler: &mut Crawler, url: &str, recursion_depth: i32, max_recursion_depth:i32, url_max: i32) {
    /*
    
        This function is intended for crawling wikipedia. It will open the url, add it to the crawler's hash set, and then it will run itself on all of the page's outgoing links 
        
    
    */

    // Because the web is so vast, it will easily reach a massive recursion depth, without something preventing that from happening. 
    // This ensures that it does not surpass a certain given recursion depth
    if recursion_depth >= max_recursion_depth {
        //println!("Maximum recursion depth exceeded.");
        return 
    }

    // If the maximum number of urls has been gathered by the crawler, then the function will just return so that the process can stop. 
    if (crawler.set.len() as i32) >= url_max {
        return 
    }

    // Fetch the HTML content of the URL 
    match fetch_html_content(url).await {
        Ok(html_content) => {

            // get the links to other wikipedia pages from the link
            let parse_result = parse::extract_wikipedia_HTML(&html_content, url);

            

            
            // Insert the current url into the crawler's history
            crawler.set.insert(url.to_string());
            
            // Information about the page being crawled
            println!("Crawled page:");
            println!("page: {}, depth: {}, page links: {}, image links: {}, url: {}", 
                crawler.set.len(),
                recursion_depth+1,
                parse_result.relevant_page_links.len(),
                parse_result.relevant_image_links.len(),
                url, 
            );

            // Here is where we can do things with the image links
            // They should be upserted to the client
            let mut itr = 0;
            for image_link in parse_result.relevant_image_links.iter() {
                if itr >= 9 { break; }
                itr+=1;
                
                let upsert_res = send_image_url(image_link,url);
                match upsert_res.await {
                    Ok(message) => {
                        // print out the status of the upsert, and the link for the image that was sent
                        println!("\tupsert: {}  {}", message, image_link);
                    }
                    Err(err) => {
                        println!("Error fetching upsert response: {}", err);
                    }
                }
            }

            // Save the crawl history to a new file
            if crawler.set.len() % 500 == 0  {

                match get_next_crawler_checkpoint_name() {
                    Some(checkpoint_name) => {
                        println!("writing crawler history to {}", checkpoint_name);
                        crawler.bincode_save(&checkpoint_name);
                    },
                    None => {
                        println!("failed to generate new checkpoint name!");
                    }
                }

            }


            // iterate through each of the page links in the wikipedia html
            for link in parse_result.relevant_page_links.iter() {
                // make sure that the given link has not already been visited by the crawler
                if !crawler.set.contains(link) {
                    // perform the recursive function
                    recursive_page_crawl(
                        crawler,
                        link,
                        recursion_depth+1, 
                        max_recursion_depth,
                        url_max,
                    ).await;
                }
            }
            
        }
        Err(err) => {
            println!("Error fetching html content: {}", err);
        }
    }
    
}


use bincode::{config, Decode, Encode};
use std::fs;
use std::path::Path;

#[derive(Decode, Encode)]
struct Crawler {
    /*
    
        This will store everything that the crawler dynamically uses as it searches the web. 
    
    */


    // the hashset contains the urls of every web page that the crawler has visited. Every time that the crawler opens a web page, it will check if the url is in the hashset beforehand, to make sure that it is not visiting a page that has already been visited
    set : HashSet<String>,


}

pub async fn initialize_crawl() {
    // This is the url where the recursive crawl is gathered from
    let start_url = "https://wikipedia.org/wiki/Google_Search";

    // The maximium nunber of pages to add to the index
    let url_max = 100000;

    // The maximum recursion depth for the crawler
    let max_recursion_depth = 100;

    // place where the crawl data is stored
    let crawler_path = "crawl_history/crawl_19";


    let bincode_config = config::standard();

    let mut crawler = Crawler {
        set: HashSet::new()
    };

    // check if a previous crawl file exists, and if it does, load it
    if Path::new(crawler_path).is_file() {

        // If the previous crawl information exists, then this will open the previous crawl binary

        println!("Found previous crawl");

        
        let crawler_binary = fs::read(crawler_path).expect("Unable to read previous crawl binary from disk");
        let (mut _crawler, len) : (Crawler, usize) = bincode::decode_from_slice(&crawler_binary[..], bincode_config).unwrap();
        println!("loaded previous crawl. contains {} urls", _crawler.set.len());

        //for link in &_crawler.set {
        //    println!("{link}");
        //}
        crawler = _crawler;
    } else {  println!("No previous crawl found, creating new crawler..."); }
    
    
    println!("Initializing recursive crawl...");

    // Start recursively crawling the web
    recursive_page_crawl(
        &mut crawler,
        start_url, 
        0,
        max_recursion_depth,
        url_max,
    ).await;


    println!("Crawling process finished. Crawl contains: {} urls", crawler.set.len());

   
}

impl Crawler {
    fn bincode_save(&self, crawler_path: &str) {
        let bincode_config = config::standard();
    
        // Save the crawler set
        let encoded_crawler : Vec<u8> = bincode::encode_to_vec(self, bincode_config).unwrap();
        fs::write(crawler_path, encoded_crawler).expect("Unable to write binary crawl file to disk");
    
        println!("Crawl history written to disk.")
    }
}


fn bincode_write_crawler(crawler: &Crawler, crawler_path: &str) {
    let bincode_config = config::standard();

    // Save the crawler set
    let encoded_crawler : Vec<u8> = bincode::encode_to_vec(crawler, bincode_config).unwrap();
    fs::write(crawler_path, encoded_crawler).expect("Unable to write binary crawl file to disk");

    println!("Crawl history written to disk.")
}

fn get_next_crawler_checkpoint_name() -> Option<String> {
     // Read the directory and handle possible errors
     match fs::read_dir("crawl_history") {
        Ok(entries) => {
            // Find the largest 'n'
            let largest_n = entries
                .filter_map(|entry| {
                    entry.ok().and_then(|dir_entry| {
                        dir_entry
                            .file_name()
                            .to_str()
                            .and_then(|file_name| parse_crawl_number(file_name))
                    })
                })
                .max()
                .unwrap_or(0);

            // Create a new string with 'crawl_{n+1}'
            let new_string = format!("crawl_history/crawl_{}", largest_n + 1);
            return Some(new_string);
        }
        Err(err) => {
            eprintln!("Error reading directory: {}", err);
            None 
        }
    }
}

// Helper function to parse the 'crawl' number from a file name
fn parse_crawl_number(file_name: &str) -> Option<u32> {
    if file_name.starts_with("crawl_") {
        file_name["crawl_".len()..].parse().ok()
    } else {
        None
    }
}
