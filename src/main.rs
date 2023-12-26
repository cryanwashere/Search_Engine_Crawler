use reqwest;
use tokio;


struct WebPageNode {
    /*

    An object representation of a web page
    
    */

    url: String,
    outgoing_node_urls: [String]
}


struct WebPageGraph {

    /*
    
        A directed graph that represents web pages, and how they connect to each other. This is generated through the scraping process, and used to calculate the page rank of each web page. 
    
    */

    nodes: Vec<WebPageNode>,
}

fn node_from_url(graph: &WebPageGraph, url: String) -> &WebPageNode {
    for node in WebPageGraph.iter() {
        if (node.url == url) {
            return *node;
        }
        // If the url was not found
        return None
    }
}


struct TransitionMatrixEdge {
    /*
    
        This populates the rows of the transition matrix
    
    */

    outgoing_node: &WebPageNode,
    incoming_node: &WebPageNode,
     

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

            };
        }

    }
}





fn main() 
{
   println!("Hello, World!");
}