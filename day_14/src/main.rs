use std::{fs, collections::HashMap};

fn main() {
    let (input, rules) = read_from_input("input");
    println!("Part 1: {}", solve(&input, &rules, 10));
    println!("Part 2: {}", solve(&input, &rules, 40));
}

fn read_from_input(filename: &str) -> (String, HashMap<String, (String, String)>) {
    let content: String = fs::read_to_string(filename).expect("Read file error");
    let (input, rules_str) = content.split_once("\n\n") .unwrap();

    let rules: HashMap<String, (String, String)> = rules_str.split('\n')
        .filter(|row| !row.is_empty())
        .map(|row| {
            let first: String = format!("{}{}", &row[0..1], &row[6..]);
            let second: String = format!("{}{}", &row[6..], &row[1..2]);
            (row[0..2].to_string(), (first, second))
        })
        .collect();

    (input.to_string(), rules)
}

fn solve(input: &str, rules: &HashMap<String, (String, String)>, step: usize) -> usize {
    let mut counter: HashMap<&str, usize> = HashMap::new();

    // init split strings
    for idx in 0..input.len()-1 {
        let entry = counter.entry(&input[idx..idx+2]).or_insert(0);
        *entry += 1;
    }

    // Iter insert
    for _ in 0..step {
        let mut new_counter: HashMap<&str, usize> = HashMap::new();

        counter.iter().for_each(|(key, val)| {
            match rules.get(*key) {
                Some((first, second)) => {
                let entry = new_counter.entry(first).or_insert(0);
                *entry += *val;
                    
                let entry = new_counter.entry(second).or_insert(0);
                *entry += *val;
                },
                None => {},
            }
        });

        counter = new_counter;
    }

    // Calculate elements
    let mut element_counter: HashMap<char, usize> = HashMap::new();
    counter.iter().for_each(|(key, val)| {
        key.chars().for_each(|ch| {
            let entry = element_counter.entry(ch).or_insert(0);
            *entry += *val;
        });
    });

    // Fill start & end input char
    let entry = element_counter.entry(input.chars().next().unwrap()).or_insert(0);
    *entry += 1;
    let entry = element_counter.entry(input.chars().last().unwrap()).or_insert(0);
    *entry += 1;

    let max: usize = *element_counter.values().max().unwrap();
    let min: usize = *element_counter.values().min().unwrap();

    (max - min) / 2
}
