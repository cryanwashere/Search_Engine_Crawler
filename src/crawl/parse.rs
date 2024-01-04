/*

    This script is meant to contain methods used to parse html.



*/


use scraper::{Html, Selector};
use std::fs;

pub fn extract_relevant_wikipedia_links(html_content: &str) -> Vec<String> {
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



pub struct HTMLExtractionResult {
    /*
    
        Entire HTML pages will be extracted in one function, and their result will be returned as this struct. This contains all of the relevant information from a HTML page that the parser extracts
    
    */

    // Relevant links to other web pages
    pub relevant_page_links: Vec<String>,

    // Relevant image links in the page
    pub relevant_image_links: Vec<String>,
}

impl HTMLExtractionResult {
    pub fn print_info(&self) {
        /*
        Prints the relevant information about the extraction
        */
        println!("Relevant Page Links: ");
        for l in self.relevant_page_links.iter() {
            println!("{}",l);
        }
        println!("Relevant Image Links ");
        for l in self.relevant_image_links.iter() {
            println!("{}", l);
        }
    }
    pub fn make_content_html(&self, content_path: &str) {
        /*
            Makes an HTML page with all of the relevant images extracted from the page
        */
        let mut out_html = String::new();

        for image_link in self.relevant_image_links.iter() {
            out_html.push_str("<img src='");
            out_html.push_str(image_link);
            out_html.push_str("'>");
        }
        fs::write(content_path, out_html).expect("failed to write HTMLExtractResult content page");
    }
}

pub fn extract_wikipedia_HTML(html_content: &str, url: &str) -> HTMLExtractionResult 
{
    /*
    
        Parse the HTML for a wikipedia page, and return an HTMLExtractionResult containing all the revelvant information stored in the page
    
    */

    // Parse the HTML
    let document = Html::parse_document(&html_content);


    // Extract all of the page links in the web page

    // create a selector for the link tags
    let a_selector = Selector::parse("a").expect("failed to parse CSS selector");

    // Array to hold all of the relevant page links
    let mut relevant_page_links: Vec<String> = Vec::new();
 
    // iterate through all the anchor tags and extract the href attribute
    for element in document.select(&a_selector) {
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
             relevant_page_links.push(wikipedia_url);
        }
    }


    // Extract all of the image links in the page

    // create a selector for image tags
    let img_selector = Selector::parse("img").expect("failed to parse CSS selector");

    // Array to hold all of the relevant page links
    let mut relevant_image_links : Vec<String> = Vec::new();

    // iterate through all of the image tags 
    for element in document.select(&img_selector) {
        // check the dimensions of the image 
        // (we filter out images that are too small)
        if let Some(width) = element.value().attr("width") {
            if width.parse::<i32>().unwrap() < 50 {
                continue;
            }
        }
        if let Some(height) = element.value().attr("height") {
            if height.parse::<i32>().unwrap() < 50 {
                continue;
            }
        }

        if let Some(src) = element.value().attr("src") {

            // Filter the link 

            // get rid of things that are in every wikipedia page
            if src.contains("static") {
                continue;
            }
            // filter out the wikipedia logo
            if src.contains("wikipedia") && src.contains("logo") {
                continue;
            }
            // get rid of pictures of math equations
            if src.contains("math/render/svg") {
                continue; 
            }

            if !src.contains("http") {
                let mut _src = "https:".to_string();
                _src.push_str(src);
                relevant_image_links.push(_src);
            }
            
            
        }
    }

    return HTMLExtractionResult{
        relevant_image_links,
        relevant_page_links,
    }

}
