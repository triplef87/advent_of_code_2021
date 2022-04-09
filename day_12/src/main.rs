use std::{collections::HashMap, fs};

enum Cave {
    Start,
    End,
    Big,
    Small
}

struct Path {
    current: usize,
    visited: Vec<bool>
}

fn main() {
    let (dots, edges) = read_from_input("input");
    let graph: Vec<Vec<bool>> = gen_graph(&edges, dots.len());

    println!("Part 1: {}", part_1(&dots, &graph));
    println!("Part 2: {}", part_2(&dots, &graph));
}

fn read_from_input(filename: &str) -> (Vec<Cave>, Vec<(usize, usize)>) {
    let mut dots: Vec<Cave> = Vec::new();
    let mut name_map: HashMap<String, usize> = HashMap::new();
    let mut edges: Vec<(usize, usize)> = Vec::new();

    fs::read_to_string(filename).expect("Read file error!")
        .split('\n')
        .filter(|row| !row.is_empty())
        .for_each(|row| {
            let (name_1, name_2): (&str, &str) = row.split_once('-').unwrap();

            let idx_1 = get_name_idx(name_1, &mut dots, &mut name_map);
            let idx_2 = get_name_idx(name_2, &mut dots, &mut name_map);

            edges.push((idx_1, idx_2));
        });
    
    (dots, edges)
}

fn get_name_idx(name: &str, dots: &mut Vec<Cave>, name_map: &mut HashMap<String, usize>) -> usize {
    match name_map.get(name) {
        Some(idx) => *idx,
        None => {
            let idx = dots.len();
            name_map.insert(name.to_string(), idx);
            if name == "start" {
                dots.push(Cave::Start);
            } else if name == "end" {
                dots.push(Cave::End);
            } else if name.chars().next().unwrap().is_uppercase() {
                dots.push(Cave::Big);
            } else {
                dots.push(Cave::Small);
            }

            idx
        },
    }
}

fn gen_graph(edges: &[(usize, usize)], len: usize) -> Vec<Vec<bool>> {
    let mut graph: Vec<Vec<bool>> = vec![vec![false; len]; len];
    edges.iter().for_each(|(i, j)| {
        graph[*i][*j] = true;
        graph[*j][*i] = true;
    });

    graph
}

fn part_1(dots: &[Cave], graph: &[Vec<bool>]) -> usize {
    let mut count = 0;

    let start_idx: usize = dots.iter().position(|dot| matches!(dot, Cave::Start)).unwrap();
    let mut stack: Vec<Path> = vec![Path { current: start_idx, visited: vec![false; dots.len()] }];

    while !stack.is_empty() {
        let cur_path: Path = stack.pop().unwrap();

        for (idx, val) in graph[cur_path.current].iter().enumerate() {
            if !val { continue; }

            match dots[idx] {
                Cave::Start => {},
                Cave::End => count += 1,
                Cave::Big => {
                    let mut clone_visited: Vec<bool> = cur_path.visited.clone();
                    clone_visited[cur_path.current] = true;

                    stack.push(Path { current: idx, visited: clone_visited });
                },
                Cave::Small => {
                    if !cur_path.visited[idx] {
                        let mut clone_visited: Vec<bool> = cur_path.visited.clone();
                        clone_visited[cur_path.current] = true;

                        stack.push(Path { current: idx, visited: clone_visited });
                    }
                },
            }
        }
    }

    count
}

fn part_2(dots: &[Cave], graph: &[Vec<bool>]) -> usize {
    let mut count = 0;

    let start_idx: usize = dots.iter().position(|dot| matches!(dot, Cave::Start)).unwrap();
    let mut stack: Vec<(Path, bool)> = vec![(Path { current: start_idx, visited: vec![false; dots.len()] }, false)];

    while !stack.is_empty() {
        let (cur_path, twice): (Path, bool) = stack.pop().unwrap();

        for (idx, val) in graph[cur_path.current].iter().enumerate() {
            if !val { continue; }

            match dots[idx] {
                Cave::Start => {},
                Cave::End => count += 1,
                Cave::Big => {
                    let mut clone_visited: Vec<bool> = cur_path.visited.clone();
                    clone_visited[cur_path.current] = true;

                    stack.push((Path { current: idx, visited: clone_visited }, twice));
                },
                Cave::Small => {
                    if !cur_path.visited[idx] {
                        let mut clone_visited: Vec<bool> = cur_path.visited.clone();
                        clone_visited[cur_path.current] = true;

                        stack.push((Path { current: idx, visited: clone_visited }, twice));
                    } else if !twice {
                        let mut clone_visited: Vec<bool> = cur_path.visited.clone();
                        clone_visited[cur_path.current] = true;

                        stack.push((Path { current: idx, visited: clone_visited }, true));
                    }
                },
            }
        }
    }

    count
}
