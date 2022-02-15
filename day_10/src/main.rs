use std::fs;

fn main() {
    let lines: Vec<String> = read_from_file("input");

    let (result_1, incomplete) = part_1(&lines);
    println!("Part 1: {result_1}");
    println!("Part 2: {}", part_2(&incomplete));
}

fn read_from_file(file_path: &str) -> Vec<String> {
    fs::read_to_string(file_path).expect("Read file fail!")
        .split('\n')
        .filter(|row| !row.is_empty())
        .map(|row| row.to_string())
        .collect()
}

fn part_1(lines: &[String]) -> (usize, Vec<Vec<char>>) {
    let mut cnt_arr: Vec<usize> = vec![0; 4];
    let mut incomplete: Vec<Vec<char>> = Vec::new();

    lines.iter().for_each(|line| {
        let mut stack: Vec<char> = Vec::new();
        let mut corrupt: bool = false;

        for ch in line.chars() {
            match ch {
                '(' | '[' | '{' | '<' => stack.push(ch),
                ')' => {
                    let pop_val = stack.pop();

                    if pop_val.is_none() || pop_val != Some('(') {
                        corrupt = true;
                    }
                },
                ']' => {
                    let pop_val = stack.pop();

                    if pop_val.is_none() || pop_val != Some('[') {
                        corrupt = true;
                    }
                },
                '}' => {
                    let pop_val = stack.pop();

                    if pop_val.is_none() || pop_val != Some('{') {
                        corrupt = true;
                    }
                },
                '>' => {
                    let pop_val = stack.pop();

                    if pop_val.is_none() || pop_val != Some('<') {
                        corrupt = true;
                    }
                },
                _ => {}
            }

            if corrupt {
                match ch {
                    ')' => cnt_arr[0] += 1,
                    ']' => cnt_arr[1] += 1,
                    '}' => cnt_arr[2] += 1,
                    '>' => cnt_arr[3] += 1,
                    _ => {}
                }
                break;
            }
        }
        
        if !corrupt {
            incomplete.push(stack);
        }
    });

    (cnt_arr[0] * 3 + cnt_arr[1] * 57 + cnt_arr[2] * 1197 + cnt_arr[3] * 25137, incomplete)
}

fn part_2(incomplete: &[Vec<char>]) -> usize {
    let mut scores: Vec<usize> = Vec::new();

    incomplete.iter().for_each(|line| {
        let mut score = 0;
        line.iter().rev().for_each(|ch| {
            score *= 5;

            match ch {
                '(' => score += 1,
                '[' => score += 2,
                '{' => score += 3,
                '<' => score += 4,
                _ => {}
            }
        });

        scores.push(score);
    });
    
    scores.sort_unstable();
    scores[scores.len()/2]
}

#[test]
fn test() {
    let lines: Vec<String> = read_from_file("test");

    let (result_1, incomplete) = part_1(&lines);
    assert_eq!(26397, result_1);
    assert_eq!(288957, part_2(&incomplete));
}
