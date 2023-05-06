use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::BufRead;
use petgraph::graph::UnGraph;
use petgraph::algo::{dijkstra};

fn read_files(dir: &str) ->UnGraph<i32, ()> {
    let paths = fs::read_dir(dir).unwrap();
    let mut graph = UnGraph::<i32, ()>::new_undirected();

    for path in paths {
        let path_str = path.unwrap().path();
        let file = fs::File::open(path_str).unwrap();
        let lines = io::BufReader::new(file).lines();
        for line in lines {
            if let Ok(content) = line {
                let contents: Vec<&str> = content.split(":").collect();
                let id: i32 = contents[0].parse().unwrap();
                let friends_vec = contents[1].split(" ");
                let idx = graph.add_node(id);
                for friend in friends_vec {
                    if let Ok(friend_id) = friend.parse() {
                        let frdx = graph.add_node(friend_id);
                        graph.add_edge(idx, frdx, ());
                    }
                }
            }
        }
    }
    graph
}

fn main() {

    let graph = read_files(".\\egonets");
    let mut distance_map = HashMap::new();
    println!("Graph Read Successfully. Total Nodes: {:?}", graph.node_count());
    println!("Calculating distance between vertices...");
    for node in graph.node_indices() {
        // println!("Finding distance for node: {:?}", node);
        let node_map = dijkstra(&graph, node, None, |_| 1);
        for (key, value) in node_map.iter() {
            distance_map.insert((node.clone(), key.clone()), value.clone());
        }
    }

    let values: Vec<&i32> = distance_map.values().collect();

    let mut times = HashMap::new();

    // count
    for x in values {
        let cnt = times.entry(*x as usize).or_insert(0);
        *cnt += 1;
    }

    let mut best: (i32, i32) = (*times.iter().nth(0).expect("Fatal.").0 as i32, *times.iter().nth(0).expect("Fatal.").1 as i32);

    for x in times.iter() {
        if *x.1 > best.1 {
            best = (*x.0 as i32, *x.1);
        }
    }
    let mode = best.0;

    println!("Usual distance between pairs of vertices is: {:?}", mode);
}
