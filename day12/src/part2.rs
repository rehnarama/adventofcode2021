use petgraph::dot::{Config, Dot};
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::EdgeDirection;
use std::collections::HashMap;

type Edge = (u32, u32);

struct CaveGraph {
    map: UnGraph<u32, ()>,
    node_map: HashMap<String, u32>,
    index_map: HashMap<u32, String>,
}
impl CaveGraph {
    fn new(input: String) -> CaveGraph {
        let edges_string: Vec<(String, String)> = input
            .lines()
            .map(|line| line.split_once("-").unwrap())
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect();

        let mut node_map = HashMap::new();
        let mut index_map = HashMap::new();
        let mut counter = 0;
        edges_string
            .iter()
            .flat_map(|(a, b)| vec![a, b])
            .cloned()
            .for_each(|node| {
                if !node_map.contains_key(&node) {
                    node_map.insert(node.clone(), counter);
                    index_map.insert(counter, node.clone());
                    counter += 1;
                }
            });

        let edges: Vec<(u32, u32)> = edges_string
            .iter()
            .map(|(from, to)| (*node_map.get(from).unwrap(), *node_map.get(to).unwrap()))
            .collect();

        let map = UnGraph::<u32, ()>::from_edges(&edges);

        CaveGraph {
            map,
            node_map,
            index_map,
        }
    }

    fn find_paths_to_end_from_start(&self) -> Vec<Vec<Edge>> {
        let start_node_index = *self.node_map.get("start").unwrap();

        let paths: Vec<Vec<Edge>> = self
            .map
            .neighbors(start_node_index.into())
            .map(|neighbour_index| (start_node_index, neighbour_index.index() as u32))
            .flat_map(|current_path| self.find_paths_to_end(vec![current_path]))
            .collect();

        paths
    }

    fn find_paths_to_end(&self, current_path: Vec<Edge>) -> Vec<Vec<Edge>> {
        let last_nodeindex = current_path.last().unwrap().1;
        let node = self.index_map.get(&last_nodeindex).unwrap();
        if node == "end" {
            return vec![current_path];
        }

        // let mut neighbours = Vec::new();
        // Create starting paths
        // for edge in neighbours {
        //     dbg!(edge);

        // let edge: Edge = (from, to);
        // neighbours.push(edge);
        // }

        let neighbours = self.map.neighbors(last_nodeindex.into());

        let mut paths = Vec::new();
        for neighbour in neighbours {
            let neighbour_index = neighbour.index() as u32;
            let node = self.index_map.get(&neighbour_index).unwrap();
            let is_small_cave = node.chars().all(char::is_lowercase);

            let node_is_start = node == "start";

            let exists_in_path = current_path.iter().fold(0, |times, (_, to)| {
                if *to == neighbour_index {
                    times + 1
                } else {
                    times
                }
            });

            let small_caves_in_path: Vec<u32> = current_path
                .iter()
                .map(|(_from, to)| to)
                .filter(|to| {
                    self.index_map
                        .get(to)
                        .unwrap()
                        .chars()
                        .all(char::is_lowercase)
                })
                .cloned()
                .collect();
            let mut deduped_small_caves_in_path: Vec<u32> = small_caves_in_path.clone();
            deduped_small_caves_in_path.sort();
            deduped_small_caves_in_path.dedup();

            let has_visisted_small_cave_twice =
                small_caves_in_path.len() != deduped_small_caves_in_path.len();

            if node_is_start
                || is_small_cave && exists_in_path >= 2
                || has_visisted_small_cave_twice && is_small_cave && exists_in_path == 1
            {
                continue;
            }

            let mut new_path = current_path.clone();
            new_path.push((last_nodeindex, neighbour_index));

            let mut branching_paths = self.find_paths_to_end(new_path);
            paths.append(&mut branching_paths);
        }

        paths
    }

    fn print_edge(&self, edge: Edge) {
        let from = self.index_map.get(&edge.0).unwrap();
        let to = self.index_map.get(&edge.1).unwrap();
        print!("{}->{}", from, to);
    }

    fn print_path(&self, path: Vec<Edge>) {
        for edge in path.iter() {
            let from = self.index_map.get(&edge.0).unwrap();
            print!("{},", from);
        }
        let last = path.last().unwrap();
        let to = self.index_map.get(&last.1).unwrap();
        print!("{}", to);
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let graph = CaveGraph::new(input.to_string());

    let paths = graph.find_paths_to_end_from_start();

    let number_of_paths = paths.len();
    dbg!(number_of_paths);
}
