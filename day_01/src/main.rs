use std::fs;

fn main() {
    let report: Vec<usize> = fs::read_to_string("input").expect("Read file error!")
        .split("\n")
        .filter(|row| !row.is_empty())
        .map(|row| row.parse::<usize>().unwrap())
        .collect();

    println!("Part 1: {}", part_1(&report));
    println!("Part 2: {}", part_2(&report));
}

fn part_1(report: &Vec<usize>) -> usize {
    let mut prev: usize = usize::MAX;

    report.iter().fold(0, |count, measurement| {
        if *measurement > prev {
            prev = *measurement;
            count + 1
        } else {
            prev = *measurement;
            count
        }
    })
}

fn part_2(report: &Vec<usize>) -> usize {
    let mut idx: usize = 0;
    let mut count: usize = 0;

    for compare_idx in 3..report.len() {
        if report[idx] < report[compare_idx] { count = count + 1; }
        idx = idx + 1;
    }
    
    count
}

#[test]
fn test_part_1() {
    let input: Vec<usize> = vec![
        199,
        200,
        208,
        210,
        200,
        207,
        240,
        269,
        260,
        263
    ];

    assert_eq!(7, part_1(&input));
}

#[test]
fn test_part_2() {
    let input: Vec<usize> = vec![
        199,
        200,
        208,
        210,
        200,
        207,
        240,
        269,
        260,
        263
    ];

    assert_eq!(5, part_2(&input));
}
