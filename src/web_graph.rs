
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/*

    Web Page Graph:
    WebPageNode, and WebPageGraph are meant to create a representation of the connections between web pages. They are not meant to be hold any information other then the connections between web pages, and they will be used to contain crawling data, and calculate things like page rank. 


*/

#[derive(Serialize, Deserialize)]
struct WebPageNode {
    /*

    An object representation of a web page
    
    */

    url: String,
    linked_urls: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct WebPageGraph {

    /*
    
        A directed graph that represents web pages, and how they connect to each other. This is generated through the scraping process, and used to calculate the page rank of each web page.
        
        The graph and node objects are meant to only contain the urls, and be as light as possible. The only thing the graph is used for is to do things that involve the connections between web pages
    
    */

    node_hashmap: HashMap<String, WebPageNode>,
}