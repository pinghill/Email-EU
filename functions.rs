
use std::collections::{HashMap, HashSet, VecDeque};
use rand::seq::SliceRandom;
//import necesarry crates


//function is to finding the shortest path distance between nodes 
pub fn shortest_path(
    graph: &HashMap<usize, HashSet<usize>>,
    start: usize,
    end: usize,
) -> Option<usize> {
    let mut visited = HashMap::new();
    //create visited hasmap, keeping track of visited nodes and distances
    let mut queue = VecDeque::new();
    //add starting node to 'visited' with distance of 0, and add to queue 
    visited.insert(start, 0);
    queue.push_back(start);

    //While nodes are in the queu, dequeue first node and visit neighbors
    while let Some(current) = queue.pop_front() {
        if current == end { //if current node is end node, return shortest path distance 
            return visited.get(&current).cloned();
        }
//Otherwise, visit the neighbors of the current node 
        if let Some(neighbors) = graph.get(&current) {
            for neighbor in neighbors {
                //if neighbor not yet visited, add to queue and set the distance 
                if !visited.contains_key(neighbor) {
                    queue.push_back(*neighbor);
                    visited.insert(*neighbor, visited[&current] + 1);
                }
            }
        }
    }
// if no path from start to end, return None
    None
}

//function is to find the average shortest path length 
pub fn shortest_path_length(graph: &HashMap<usize, HashSet<usize>>) -> f64 {
    let mut total_length = 0;
    let mut num_paths = 0;
    //initialze variables to store total_length and num_paths

    for &start in graph.keys() {
        for &end in graph.keys() {
            if start != end {
                //If there is a shortest path, add length to total and incriment num_paths by 1
                if let Some(length) = shortest_path(graph, start, end) {
                    total_length += length;
                    num_paths += 1;
                }
            }
        }
    }
//return the average shortest path length 
    (total_length as f64) / (num_paths as f64)
}

//function is to find the top 5 nodes with incoming edges 
pub fn print_top_5(graph: &HashMap<usize, HashSet<usize>>) {
    //initialize hashmap to keep track of number of incoming edges for each node 
    let mut incoming_edges_count: HashMap<usize, usize> = HashMap::new();

    //iterate through each node and edges 
    for (_node, edges) in graph.iter() {
        //for each edge, incriment the count of number of incoming edges for the corresponding node 
        for edge in edges {
            *incoming_edges_count.entry(*edge).or_insert(0) += 1;
        }
    }
//convert incoming_ednge_count map to a vector of (node, count), and sort by decreasing order 
    let mut nodes_by_incoming_edges: Vec<(usize, usize)> = incoming_edges_count.into_iter().collect();
    nodes_by_incoming_edges.sort_unstable_by(|a, b| b.1.cmp(&a.1));
//Print top 5 nodes with most incoming edges 
    println!("Top 5 nodes with the most incoming edges:");
    for (i, (node, count)) in nodes_by_incoming_edges.iter().take(5).enumerate() {
        println!("{}. Node {}: {} incoming edges", i + 1, node, count);
    }
}
