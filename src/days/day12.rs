use std::collections::HashMap;
use std::fs;

pub fn solve() -> (String, String) {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut visited: HashMap<&str, bool> = HashMap::new();
    let stream = fs::read_to_string("inputs/day12.txt").unwrap();
    for i in stream.trim().lines() {
        let v: Vec<&str> = i.split("-").collect();
        visited.insert(v[0], false);
        visited.insert(v[1], false);
        graph.entry(v[0]).or_insert(Vec::new()).push(v[1]);
        if v[0] != "start" && v[1] != "end" {
            graph.entry(v[1]).or_insert(Vec::new()).push(v[0]);
        }
    }

    let part1_sol = part1(&graph, &visited);
    let part2_sol = part2(&graph, &visited);

    (part1_sol, part2_sol)
}

fn part1(graph: &HashMap<&str, Vec<&str>>, visited: &HashMap<&str, bool>) -> String {
    let mut sum = 0;
    sum += traverse(graph, "start", &mut visited.clone(), false, false);
    return sum.to_string();
}

fn part2(graph: &HashMap<&str, Vec<&str>>, visited: &HashMap<&str, bool>) -> String {
    let mut sum = 0;
    sum += traverse(graph, "start", &mut visited.clone(), true, false);
    return sum.to_string();
}

fn traverse(
    graph: &HashMap<&str, Vec<&str>>,
    node: &str,
    visited: &mut HashMap<&str, bool>,
    en_mult: bool,
    twice: bool,
) -> u32 {
    if node == "end" {
        return 1;
    }

    let mut sum = 0;
    *visited.get_mut(node).unwrap() = true;

    for n in graph.get(node).unwrap() {
        if n.to_uppercase() != *n && visited[*n] {
            if en_mult && !twice && *n != "start" {
                sum += traverse(graph, n, &mut visited.clone(), en_mult, true)
            }
        } else {
            sum += traverse(graph, n, &mut visited.clone(), en_mult, twice);
        }
    }

    return sum;
}
