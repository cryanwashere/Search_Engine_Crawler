use reqwest;
use tokio;

use std::collections::HashMap;

use scraper::{Html, Selector};

use async_recursion::async_recursion;


use serde::{Serialize, Deserialize};


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

/*
struct TransitionMatrixEdge<'a> {
    /*
    
        This populates the rows of the transition matrix
    
    */

    outgoing_node: &'a WebPageNode,
    incoming_node: &'a WebPageNode,
     

    // The probability that this edge will be traversed by a markov chain, given that it is already at 'outgoing_node'
    transition_probability: f32,

}

struct SparseTransitionMatrixRow {
    /*
    
        A row in the sparse transition matrix. It contains a list representing each of the edges and their probabilities in the row.

        Each row represents a node, and each edge inside the row contains the probability that a markov chain at the outgoing_node of the edge will traverse to the node. 
    
    */

    // This is the url of the web page that the row represents
    url: String,

    transition_edges: Vec<TransitionMatrixEdge>,
}

struct SparseMarkovTransitionMatrix {
    /*
    
        Given a graph for a web page, we want to constuct a sparse matrix that represents the transition probabilities for the web page graph. Each row of the matrix represents the probability vector of arriving at that node from the node represented by each given index. If x_n is the probability vector representing the probability of being at each specific node at step n, then the probability of being at each specific node at step n+1 is the matrix vector product of P and x_n (Where P is the transition matrix, and P(a,b) is the probability of transitioning to node b from node a): 

        x_{n+1} = Px_n = 

        [P(0)P(0,0) + P(1)P(1,0) + ... + P(m)P(m,0)]
        [P(0)P(0,1) + P(1)P(1,1) + ... + P(m)P(m,1)]
        [                   ...                    ]
        [P(0)P(0,m) + P(1)P(1,m) + ... + P(m)P(m,m)]

        =

        [P(0,0), P(1,0), ... , P(m,0)] [P(0)]
        [P(0,1), P(1,1), ... , P(m,1)] [P(1)]
        [ ...                    ... ]  ...
        [P(0,m), P(1,m), ... , P(m,m)] [P(m)]

    */
    transition_matrix_rows: Vec<SparseTransitionMatrixRow>

}

fn construct_markov_transition_matrix(graph: &WebPageGraph) -> SparseMarkovTransitionMatrix 
{
    /*
    
        Construct the markov transition matrix a web page graph. This is essentially transposing a sparse matrix. It is assumed that when a markov chain is at a specific node, it is equiprobable that it will traverse to any of the node's outgoing edges. 
    
    */

    // first, create the matrix rows. It is nescessary to create all of the rows before they are populated by web pages. 
    let mut transition_matrix_rows: Vec<SparseTransitionMatrixRow> = Vec::new();
    for node in graph.nodes.into_iter() {

        // create the row for the transition matrix
        let row = SparseTransitionMatrixRow {
            url: node.url,
            transition_edges: Vec::new(), 
        };
        transition_matrix_rows.push(row);
    }

    // Now, we iterate through the graph nodes again, and add the edges to their existing nodes. 
    for node in graph.nodes.into_iter() {
        let outgoing_nodes_i32 = node.outgoing_nodes.len();
        let outgoing_nodes_f32 = outgoing_nodes_i32 as f32;

        // iterate through the outgoing edges of the node:
        for outgoing_edge_url in node.outgoing_node_urls {
            let edge = TransitionMatrixEdge{
                outgoing_node: *node,
                incoming_node: node_from_url(outgoing_edge_url),
                transition_probability: 1/outgoing_nodes_f32,
            };
        }

    }
}

*/


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
    
        Parse the HTML content of a web page, and return a vector containing a string for each of the relevant links to the crawler 
    
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
async fn recursive_page_crawl(graph: &mut WebPageGraph, url: &str, recursion_depth: i32, max_recursion_depth:i32, url_max: i32) {
    /*
    
        
    
    */

    if recursion_depth >= max_recursion_depth {
        println!("Maximum recursion depth exceeded.");
        return 
    }

    if (graph.node_hashmap.len() as i32) >= url_max {
        //println!("Maximum number of pages crawled. Process finished.");
        return 
    }

    

    match fetch_html_content(url).await {
        Ok(html_content) => {

            // get the links to other wikipedia pages from the link
            let relevant_links = extract_relevant_wikipedia_links(&html_content);

            // insert the url into the graph
            let url_node = WebPageNode {
                url: url.to_string(),
                linked_urls: relevant_links.clone(),
            };
            graph.node_hashmap.insert(url.to_string(), url_node);
            
            println!("page: {} depth: {} links: {} {}", 
                graph.node_hashmap.len(),
                recursion_depth+1,
                relevant_links.len(),
                url, 
            );

            // iterate through each of the links in the wikipedia page
            for link in relevant_links.iter() {
                // make sure that the given link has not already been visited by the crawler
                if !graph.node_hashmap.contains_key(link) {
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

fn save_to_disk_json<T>(object: &T, filename: &str) where T: Serialize, {
    let serialized_data = serde_json::to_string(object).expect("Serialization failed");

    use std::fs::File;
    use std::io::Write;

    let mut file = File::create(filename).expect("Unable to create file");
    file.write_all(serialized_data.as_bytes()).expect("Unable to write file");
}

fn load_from_disk_json<T>(filename: &str) -> Result<T, serde_json::Error>
where
    T: Deserialize<'static>,
{
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(filename)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let deserialized_data: T = serde_json::from_str(&buffer)?;

    Ok(deserialized_data)
}

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
    //let mut web_page_graph = WebPageGraph {
        node_hashmap: HashMap::new()
    //};

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