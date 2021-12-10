use std::fs;

fn main() {
    let plan: Vec<String> = read_from_file("input");
    println!("Part 1:{}", part1(&plan));
    println!("Part 2:{}", part2(&plan));
}

fn read_from_file(path: &str) -> Vec<String> {
    fs::read_to_string(path).expect("Read file error!")
        .split("\n")
        .filter(|row| !row.is_empty())
        .map(|row| row.to_string())
        .collect()
}

fn part1(plan: &Vec<String>) -> usize {
    let mut pos: (usize, usize) = (0 , 0);

    plan.iter().for_each(|step| {
        let (direct, unit_str) = step.split_once(" ").unwrap();
        let unit: usize = unit_str.parse::<usize>().unwrap();

        match direct {
            "forward" => pos.1 = pos.1 + unit,
            "up" => pos.0 = pos.0 - unit,
            "down" => pos.0 = pos.0 + unit,
            _ => unreachable!()
        }
    });

    pos.0 * pos.1
}

fn part2(plan: &Vec<String>) -> usize {
    let mut pos: (usize, usize) = (0 , 0);
    let mut aim: usize = 0;

    plan.iter().for_each(|step| {
        let (direct, unit_str) = step.split_once(" ").unwrap();
        let unit: usize = unit_str.parse::<usize>().unwrap();

        match direct {
            "forward" => { 
                pos.1 = pos.1 + unit;
                pos.0 = pos.0 + aim*unit;
            },
            "up" => aim = aim - unit,
            "down" => aim = aim + unit,
            _ => unreachable!()
        }
    });

    pos.0 * pos.1
}

#[test]
fn test_1() {
    let plan: Vec<String> = read_from_file("test");
    assert_eq!(150, part1(&plan));
}

#[test]
fn test_2() {
    let plan: Vec<String> = read_from_file("test");
    assert_eq!(900, part2(&plan));
}