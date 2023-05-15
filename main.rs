#![allow(unused_imports)]
use std::io::Write;
use std::fs::File;
use tempfile::tempdir;
use std::collections::{HashMap, HashSet};
//import functions from 'functions' and 'read_dataset' modules

mod functions;
mod read_dataset;
//use functions from 'functions and 'read_dataset' modules 
use functions::{shortest_path_length, print_top_5};
use read_dataset::read;
use std::path::Path;

//define main funciton 
fn main() {
    //set path of input file 
    let file_path = Path::new("/Users/iphone10/Downloads/email-EuAll.txt");
    //limit the max number of nodes being processed
    let max_nodes = Some(500);
    //read graph from the input file 
    let graph = read(&file_path, max_nodes);
    //calculate the average shortest path of the graph 
    let avg_shortest_path_length = shortest_path_length(&graph);

    //print the top 5 nodes in the graph 
    print_top_5(&graph);
    //print the average shortest path length 
    println!("Average shortest path length: {:.2}", avg_shortest_path_length);
    
}
//define test for the read function 
#[test]
fn test_read() {
    // create a temporary directory
    let dir = tempdir().expect("Failed to create temp dir");
    let file_path = dir.path().join("test_data.txt");

    // write a simple test data file
    let data = "1 2\n2 3\n3 1";
    let mut file = File::create(&file_path).expect("Failed to create test data file");
    file.write_all(data.as_bytes()).expect("Failed to write test data");

    // call the read function with the test data file
    let result = read(file_path.as_path(), None);

    // check if the returned graph is correct
    let mut expected: HashMap<usize, HashSet<usize>> = HashMap::new();
    expected.insert(3, [1].iter().cloned().collect());
    expected.insert(2, [3].iter().cloned().collect());
    expected.insert(1, [2].iter().cloned().collect());

    assert_eq!(result, expected);
}
//define test for node count 
#[test]
fn test_node_count() {
    //set the path of the input data
    let file_path = Path::new("/Users/iphone10/Downloads/email-EuAll.txt");
    //set the maximum number of nodes to read 
    let max_nodes = Some(265214);
    //read the graph from the input data
    let graph = read(&file_path, max_nodes);
//check if the number of nodes in the graph is correct 
    let expected_node_count = 265214;
    let actual_node_count: usize = graph.len();

    assert!(
        expected_node_count >= actual_node_count,
        "Node count in the graph is incorrect."
    );
}









