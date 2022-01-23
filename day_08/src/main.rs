use std::{fs, collections::HashSet};

struct Entry {
    pattern: Vec<HashSet<char>>,
    output: Vec<HashSet<char>>
}

impl Entry {
    fn parse_row(row: &str) -> Entry {
        let (pattern, output) = row.split_once(" | ").unwrap();

        let pattern: Vec<HashSet<char>> = pattern.split(' ').map(|ele| ele.chars().collect()).collect();
        let output: Vec<HashSet<char>> = output.split(' ').map(|ele| ele.chars().collect()).collect();

        Entry { pattern, output }

    }
}

fn main() {
    let note: Vec<Entry> = read_from_file("input");
    
    println!("Part 1: {}", part_1(&note));
    println!("Part 2: {}", part_2(&note));
}

fn read_from_file(file_path: &str) -> Vec<Entry> {
    fs::read_to_string(file_path).expect("Read file error")
        .split('\n')
        .filter(|row| !row.is_empty())
        .map(Entry::parse_row)
        .collect()
}

fn part_1(note: &[Entry]) -> usize {
    let mut count = 0;

    note.iter().for_each(|entry| {
        entry.output.iter().for_each(|digits| {
            if digits.len() == 2 || digits.len() == 3 || digits.len() == 4 || digits.len() == 7 {
                count += 1;
            } 
        });
    });

    count
}

fn part_2(note: &[Entry]) -> usize {
    let mut sum = 0;

    note.iter().for_each(|entry| {
        let mut base_digit: [usize; 2] = [0; 2];

        entry.pattern.iter().enumerate().for_each(|(idx, digits)| {
            match digits.len() {
                2 => base_digit[0] = idx, // digit 1
                4 => base_digit[1] = idx, // digit 4
                _ => {}
            }
        });

        let mut val = 0;
        entry.output.iter().enumerate().for_each(|(idx, digits)| {
            let mut digit = 0;
            match digits.len() {
                2 => digit = 1,
                3 => digit = 7,
                4 => digit = 4,
                5 => {
                    if digit_diff(&entry.pattern[base_digit[0]], digits) == 0 {
                        digit = 3;
                    } else if digit_diff(&entry.pattern[base_digit[1]], digits) == 2 {
                        digit = 2;
                    } else {
                        digit = 5;
                    }
                },
                6 => {
                    if digit_diff(&entry.pattern[base_digit[0]], digits) == 1 {
                        digit = 6;
                    } else if digit_diff(&entry.pattern[base_digit[1]], digits) == 0 {
                        digit = 9;
                    } else {
                        digit = 0;
                    }
                },
                7 => digit = 8,
                _ => {}
            }

            match idx {
                0 => val += digit * 1000,
                1 => val += digit * 100,
                2 => val += digit * 10,
                3 => val += digit,
                _ => {},
            }
        });

        sum += val;
    }); 

    sum
}

fn digit_diff(base: &HashSet<char>, target: &HashSet<char>) -> usize {
    let mut count = 0;

    base.iter().for_each(|ch| {
        if !target.contains(ch) { count += 1; }
    });

    count
}

#[test]
fn test_1() {
    let note: Vec<Entry> = read_from_file("test");
    assert_eq!(26, part_1(&note));
}

#[test]
fn test_2() {
    let note: Vec<Entry> = read_from_file("test");
    assert_eq!(61229, part_2(&note));
}


