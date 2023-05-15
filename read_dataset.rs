use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;


pub fn read(file_path: &Path, max_nodes: Option<usize>) -> HashMap<usize, HashSet<usize>> {
    //open file, return error if cannot be opened
    let file = File::open(&file_path).expect("Cannot open file");
    let reader = BufReader::new(file);
    //create Bufreader to read in data more efficiently
    //Initialze the graph as an empty hashmap
    let mut graph: HashMap<usize, HashSet<usize>> = HashMap::new();

    //iterate through the lines of code 
    for line in reader.lines() {
        //unwrap each line, returning error if cannot be read
        let line = line.expect("Cannot read line");
        //Checking if the line starts wtiha  comment
        if !line.starts_with("#") {
            //Splitting line of whitespace, collect nodes into a vector 
            let nodes: Vec<&str> = line.split_whitespace().collect();
            let from = nodes[0].parse::<usize>().unwrap();
            let to = nodes[1].parse::<usize>().unwrap();
//make sure the nodes are inside the parameters of max_nodes
            if let Some(max) = max_nodes {
                if from >= max || to >= max {
                    continue;
                }
            }
//inserting edge (from, to) into the graph 
            graph.entry(from).or_insert(HashSet::new()).insert(to);
            
        }
    }

    graph
}
