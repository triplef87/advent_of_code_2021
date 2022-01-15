use std::{fs, collections::HashMap};

#[derive(Debug, Eq, PartialEq, Hash)]
struct Pos {
    x: isize,
    y: isize
}

impl Pos {
    fn convert(input: &str) -> Pos {
        let (x, y) = input.split_once(',').unwrap();
        Pos { x: x.parse::<isize>().unwrap(), y: y.parse::<isize>().unwrap() }
    }
}

fn main() {
    let plan: Vec<(Pos, Pos)> = read_from_file("input");

    println!("Part 1: {}", part_1(&plan));
    println!("Part 2: {}", part_2(&plan));
}

fn read_from_file(file: &str) -> Vec<(Pos, Pos)> {
    fs::read_to_string(file).expect("Read from file error!")
        .split('\n')
        .filter(|row| !row.is_empty())
        .map(|row| {
            let (start, end) = row.split_once(" -> ").unwrap();
            (Pos::convert(start), Pos::convert(end))
        })
        .collect()
}

fn part_1(plan: &[(Pos, Pos)]) -> usize {
    let mut walked: HashMap<Pos, usize> = HashMap::new();

    for (start, end) in plan {
        let mut step: Pos = Pos { x: end.x - start.x, y: end.y - start.y };

        if step.x != 0 { step.x /= step.x.abs(); }
        if step.y != 0 { step.y /= step.y.abs(); }

        if step.x != 0 && step.y != 0 {
            continue;
        }

        let mut head: Pos = Pos { x: start.x, y: start.y };
        while head != *end {
            let tmp: Pos = Pos { x: head.x, y: head.y };
            let entry = walked.entry(tmp).or_insert(0);
            *entry += 1;
            
            head.x += step.x;
            head.y += step.y;
        }
        
        let entry = walked.entry(head).or_insert(0);
        *entry += 1;
    }

    walked.values().filter(|count| **count > 1).count()
}

fn part_2(plan: &[(Pos, Pos)]) -> usize {
    let mut walked: HashMap<Pos, usize> = HashMap::new();
    
    for (start, end) in plan {
        let diff_x: isize = end.x - start.x;
        let diff_y: isize = end.y - start.y;

        if diff_x != 0 && diff_y != 0 && diff_x.abs() != diff_y.abs() {
            continue;
        }

        let step_x = match diff_x.cmp(&0) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        };

        let step_y = match diff_y.cmp(&0) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        };

        let mut head: Pos = Pos { x: start.x, y: start.y };
        while head != *end {
            let tmp: Pos = Pos { x: head.x, y: head.y };
            let entry = walked.entry(tmp).or_insert(0);
            *entry += 1;
            
            head.x += step_x;
            head.y += step_y;
        }
        
        let entry = walked.entry(head).or_insert(0);
        *entry += 1;
    }

    walked.values().filter(|count| **count > 1).count()
}

#[test]
fn test1() {
    let plan: Vec<(Pos, Pos)> = read_from_file("test");

    assert_eq!(part_1(&plan), 5);
}

#[test]
fn test2() {
    let plan: Vec<(Pos, Pos)> = read_from_file("test");

    assert_eq!(part_2(&plan), 12);
}