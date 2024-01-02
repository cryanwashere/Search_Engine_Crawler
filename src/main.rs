

use std::collections::HashMap;
use std::collections::HashSet;




use serde::{Serialize, Deserialize};


use serde_json::Result;


use std::io;
use std::fs;


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


#[derive(Serialize, Deserialize)]
struct Crawler {
    /*
    
        This will store everything that the crawler dynamically uses as it searches the web. 
    
    */


    // the hashset contains the urls of every web page that the crawler has visited. Every time that the crawler opens a web page, it will check if the url is in the hashset beforehand, to make sure that it is not visiting a page that has already been visited
    visited_hashset = HashSet<String>




}

impl Crawler {
    
}

/*


/*

    Serialize and deserialize rust objects to JSOn. Node that these two functions were mostly written by chatGPT.

*/
fn json_write_to_disk<T>(object: &T, filename: &str) where T: Serialize, {
    let serialized_data = serde_json::to_string(object).expect("Serialization failed");

    use std::fs::File;
    use std::io::Write;

    let mut file = File::create(filename).expect("Unable to create file");
    file.write_all(serialized_data.as_bytes()).expect("Unable to write file");
}
*/



fn main() {
    println!("hello");

}