use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use super::{Day, Input, Types};

type Graph = HashMap<(String, String), u32>;
type Path = HashSet<String>;

fn process(data: &str) -> (Graph, Path) {
    let mut graph = Graph::new();
    let mut path = Path::new();
    for line in data.lines() {
        let split: Vec<&str> = line.split_ascii_whitespace().collect();
        let points = (split[0].to_string(), split[2].to_string());
        let dist = split[4].parse().unwrap();
        path.insert(points.0.clone());
        path.insert(points.1.clone());
        graph.insert(points, dist);
    }
    (graph, path)
}

fn path_lengths(graph: &Graph, paths: &Path) -> Vec<u32> {
    let mut lengths = vec![];
    for path in paths.iter().permutations(paths.len()).unique() {
        let mut len = 0;
        let mut success = false;
        for (a, b) in path.iter().tuple_windows() {
            for ((left, right), dist) in graph {
                if (a == &left && b == &right) || (b == &left && a == &right) {
                    len += dist;
                    success = true;
                    break;
                }
            }
            if !success {
                break;
            }
        }
        if success {
            lengths.push(len);
        }
    }
    lengths
}

pub fn part_a(input: &Input) -> Types {
    let (graph, paths) = process(&input.contents);
    Types::Number(
        (*path_lengths(&graph, &paths).iter().min().unwrap())
            .try_into()
            .unwrap(),
    )
}

pub fn part_b(input: &Input) -> Types {
    let (graph, paths) = process(&input.contents);
    Types::Number(
        (*path_lengths(&graph, &paths).iter().max().unwrap())
            .try_into()
            .unwrap(),
    )
}

pub static DAY: Day = Day { part_a, part_b };
