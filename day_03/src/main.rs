use std::fs;

fn main() {
    let report: Vec<Vec<bool>> = read_from_file("input");
    println!("Part 1: {}", part_1(&report));
    println!("Part 2: {}", part_2(&report));
}

fn read_from_file(path: &str) -> Vec<Vec<bool>> {
    fs::read_to_string(path).expect("Read file error!")
        .split("\n")
        .filter(|row| !row.is_empty())
        .map(|row| row.chars().map(|c| {
            if c == '1' { true } else { false }
        }).collect())
        .collect()
}

fn part_1(report: &Vec<Vec<bool>>) -> usize {
    let report_len: usize = report.len();
    let report_half: usize = report_len / 2;
    let number_len: usize = report[0].len();

    let mut common: Vec<usize> = vec![0; number_len];
    report.iter().for_each(|number| {
        number.iter().enumerate().for_each(|(idx,digit)| {
            if *digit { common[idx] = common[idx] + 1; }
        });
    });

    let mut gamma_rate: usize = 0;
    let mut epsilon_rate: usize = 0;

    for idx in 0..number_len {
        if common[idx] > report_half {
            gamma_rate = gamma_rate + (1 << (number_len-1-idx));
        } else {
            epsilon_rate = epsilon_rate + (1 << (number_len-1-idx));
        }
    }

    gamma_rate*epsilon_rate
}

fn part_2(report: &Vec<Vec<bool>>) -> usize {
    let oxygen: usize = gen(report.clone(), true);
    let co2: usize = gen(report.clone(), false); 

    oxygen*co2
}

fn gen(mut report: Vec<Vec<bool>>, flag: bool) -> usize {
    let mut idx: usize = 0;
    while report.len() > 1 {
        let len: usize = report.len();
        let mut count: usize = 0;
        report.iter().for_each(|number| if number[idx] { count = count + 1});
        
        let mut check: bool = if count*2 == len { true } else { count > len/2 };
        if !flag { check = !check; }
        report.retain(|number| number[idx] == check);
        idx = idx + 1;
    }
    
    let num_len: usize = report[0].len();
    let mut result: usize = 0;
    for idx in 0..num_len {
        if report[0][idx] { result = result + (1 << (num_len-1-idx)); }
    }

    result
}

#[test]
fn test_1() {
    let report: Vec<Vec<bool>> = read_from_file("test");
    assert_eq!(198, part_1(&report));
}

#[test]
fn test_2() {
    let report: Vec<Vec<bool>> = read_from_file("test");
    assert_eq!(230, part_2(&report));
}