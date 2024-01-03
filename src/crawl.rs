use async_recursion::async_recursion;

use reqwest;
use tokio;

use scraper::{Html, Selector};

use serde::{Serialize, Deserialize};

use std::collections::HashSet;



#[derive(Serialize, Deserialize)]
struct Crawler {
    /*
    
        This will store everything that the crawler dynamically uses as it searches the web. 
    
    */


    // the hashset contains the urls of every web page that the crawler has visited. Every time that the crawler opens a web page, it will check if the url is in the hashset beforehand, to make sure that it is not visiting a page that has already been visited
    visited_hashset : HashSet<String>,


}



async fn fetch_html_content(url: &str) -> Result<String, reqwest::Error> {
    /*

        Make an HTTP request to get the HTML content for the given URL
    
    */
    let response = reqwest::get(url).await?;
    let html_content = response.text().await?;
    Ok(html_content)
}



fn extract_relevant_wikipedia_links(html_content: &str) -> Vec<String> {
    /*
    
        Parse the HTML content of a wikipedia page, and return a vector containing a string for each of the relevant links to the crawler 
    
    */


    // parse the HTML for the web page
    let document = Html::parse_document(&html_content);

    // create a selector for the link tags
    let selector = Selector::parse("a").expect("failed to parse CSS selector");

    let mut relevant_links: Vec<String> = Vec::new();

    // iterate through all the anchor tags and extract the href attribute
    for element in document.select(&selector) {
        // extract the href attribute
        if let Some(href) = element.value().attr("href") {

            // Here the link is filtered to make sure that the function only returns desired links. Most of these criteria were created through trial and error, and do a fairly good job of only allowing links to other wikipedia pages to pass through

            if href.contains("wikipedia.org") { continue }

            if href.contains("wikidata.org") { continue }

            if href.contains("wikimedia") { continue }

            if href.contains("https://") { continue }

            if href.contains(":") { continue }

            if href.contains("#") { continue }

            if href.contains("%") { continue }

            if href.contains("&") { continue }

            if href.contains("disambiguation") { continue }

            let mut wikipedia_url = String::from("https://wikipedia.org");
            wikipedia_url.push_str(href);

            // assuming the link has passed through the 'filter'
            relevant_links.push(wikipedia_url);
        }
    }

    return relevant_links
}


#[async_recursion]
async fn recursive_page_crawl(crawler: &mut Crawler, url: &str, recursion_depth: i32, max_recursion_depth:i32, url_max: i32) {
    /*
    
        This function is intended for crawling wikipedia. 
        
    
    */

    if recursion_depth >= max_recursion_depth {
        println!("Maximum recursion depth exceeded.");
        return 
    }

    // If the maximum number of urls has been gathered by the crawler, then the function will just return so that the process can stop. 
    if (crawler.visited_hashset.len() as i32) >= url_max {
        //println!("Maximum number of pages crawled. Process finished.");
        return 
    }

    
    // Fetch the HTML content of the URL 
    match fetch_html_content(url).await {
        Ok(html_content) => {

            // get the links to other wikipedia pages from the link
            let relevant_links = extract_relevant_wikipedia_links(&html_content);

            /*
            // insert the url into the graph
            let url_node = WebPageNode {
                url: url.to_string(),
                linked_urls: relevant_links.clone(),
            };
            graph.node_hashmap.insert(url.to_string(), url_node);
            */

            crawler.visited_hashset.insert(url.to_string());
            
            println!("page: {} depth: {} links: {} {}", 
                crawler.visited_hashset.len(),
                recursion_depth+1,
                relevant_links.len(),
                url, 
            );

            // iterate through each of the links in the wikipedia page
            for link in relevant_links.iter() {
                // make sure that the given link has not already been visited by the crawler
                if !crawler.visited_hashset.contains(link) {
                    // perform the recursive function
                    recursive_page_crawl(
                        graph,
                        link,
                        recursion_depth+1, 
                        max_recursion_depth,
                        url_max,
                    ).await;
                }
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
    
}




pub async fn crawl() {
    // This is the url where the recursive crawl is gathered from
    let start_url = "https://wikipedia.org/wiki/Google_Search";

    // The maximium nunber of pages to add to the index
    let url_max = 50;

    // The maximum recursion depth for the crawler
    let max_recursion_depth = 32;

    // the crawler that holds all of the crawling data
    let mut crawler = Crawler {
        visited_hashset: HashSet::new()
    };

    println!("Initializing recursive crawl...");
    recursive_page_crawl(
        &mut web_page_graph,
        start_url, 
        0,
        max_recursion_depth,
        url_max,
    ).await;

   let filename = "crawl_1.json";

   save_to_disk_json(&web_page_graph, filename);
}


/*

#[tokio::main]
async fn main() 
{
    // This is the url where the recursive crawl is gathered from
    let start_url = "https://wikipedia.org/wiki/Google_Search";

    // The maximium nunber of pages to add to the index
    let url_max = 5;

    // The maximum recursion depth for the crawler
    let max_recursion_depth = 30;

    // the graph that stores all the links
    let mut web_page_graph = WebPageGraph {
        node_hashmap: HashMap::new()
    };

    println!("Initializing recursive crawl...");
    recursive_page_crawl(
        &mut web_page_graph,
        start_url, 
        0,
        max_recursion_depth,
        url_max,
    ).await;

   let filename = "crawl_1.json";

   save_to_disk_json(&web_page_graph, filename);

   
}
*/
