use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::BufRead;
use petgraph::graph::{UnGraph};
use petgraph::algo::{dijkstra};


fn read_files(dir: &str) ->UnGraph<i32, ()> {

    let mut graph = UnGraph::<i32, ()>::new_undirected();

    let paths = fs::read_dir(dir).unwrap();

    for path in paths {
        let path_str = path.unwrap().path();
        let file = fs::File::open(path_str).unwrap();

        let lines = io::BufReader::new(file).lines();
        for line in lines {

            if let Ok(content) = line {

                let contents: Vec<&str> = content.split(":").collect();

                let id: i32 = contents[0].parse().unwrap();
                let idx = graph.add_node(id);

                let friends_vec = contents[1].split(" ");
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

fn find_mode(values: Vec<&i32>) -> i32 {

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

    return best.0;
}

fn calculate_usual_distance(graph: UnGraph<i32, ()>){

    let mut distance_map = HashMap::new();

    println!("Calculating distance between vertices...");

    for node in graph.node_indices() {
        // println!("Finding distance for node: {:?}", node);
        let node_map = dijkstra(&graph, node, None, |_| 1);

        for (key, value) in node_map.iter() {
            distance_map.insert((node.clone(), key.clone()), value.clone());
        }
    }

    let values: Vec<&i32> = distance_map.values().collect();

    let mode = find_mode(values);
    println!("Usual distance between pairs of vertices is: {:?}", mode);

}

fn degree_of_distribution(graph: UnGraph<i32, ()>){

    let mut distribution = vec![
        vec![], vec![], vec![], vec![], vec![]
    ];

    let mut que1 = vec![];
    let mut que2 = vec![];

    for node in graph.node_indices() {

        println!("Calculating Distribution for: {:?}", node);

        for neighbour in graph.neighbors(node){
            que1.push(neighbour);
        }

        for i in 1..5 {

            if i%2==0{
                distribution[i-1].push(que2.len() as i32);

                for visit_node in que2.iter(){
                    for neighbour in graph.neighbors(*visit_node) {
                        que1.push(neighbour);
                    }
                }
            }else{
                distribution[i-1].push(que1.len() as i32);

                for visit_node in que1.iter(){
                    for neighbour in graph.neighbors(*visit_node) {
                        que2.push(neighbour);
                    }
                }
            }

        }
    }
    let mut modes = vec![];

    for counts in distribution {
        let mut converted = vec![];

        for count in counts.iter() {
            converted.push(count);
        }

        modes.push(find_mode(converted));

        println!("{:?}", modes);
    }
}

fn main() {

    let graph = read_files(".\\egonets");
    println!("Graph Read Successfully. Total Nodes: {:?}", graph.node_count());

    // usual distance between vertices
    calculate_usual_distance(graph);

    // degree distribution
    // degree_of_distribution(graph);

}
