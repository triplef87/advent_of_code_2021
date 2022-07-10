use std::{fs, collections::HashMap};

fn main() {
    let (image_ref, input): (Vec<char>, HashMap<(isize, isize), char>) = read_from_input("input");
    println!("Part 1: {}", process(&image_ref, input.clone(), 2));
    println!("Part 2: {}", process(&image_ref, input, 50));
}

fn read_from_input(filename: &str) -> (Vec<char>, HashMap<(isize, isize), char>) {
    let content: Vec<String> = fs::read_to_string(filename).expect("Read file error!")
        .split("\n\n")
        .map(|row| row.to_string())
        .collect();

    let image_ref: Vec<char> = content[0].chars().filter(|ch| *ch != '\n').collect();
    let mut input: HashMap<(isize, isize), char> = HashMap::new();
    content[1].split('\n')
        .filter(|row| !row.is_empty())
        .enumerate()
        .for_each(|(i, row)| {
            row.chars().enumerate().for_each(|(j, ch)| { input.insert((i as isize, j as isize), ch); } );
        });


    (image_ref, input)
}

fn enhance(image_ref: &[char], mut input: HashMap<(isize, isize), char>, void: &mut char) -> HashMap<(isize, isize), char> {
    let mut output: HashMap<(isize, isize), char> = HashMap::new();
    let next_void: char = if *void == '.' { image_ref[0] } else { image_ref[511] };

    for (x, y) in input.clone().keys() {
        for ref_x in (-1..=1).rev() {
            for ref_y in (-1..=1).rev() {
                output.entry((x+ref_x, y+ref_y)).or_insert(next_void);
                input.entry((x+ref_x, y+ref_y)).or_insert(*void);
            }
        }
    }

    for (x, y) in input.keys() {
        let mut idx = 0;
        let mut tmp = 1;

        for ref_x in (-1..=1).rev() {
            for ref_y in (-1..=1).rev() {
                match input.get(&(x+ref_x, y+ref_y)) {
                    None => { if *void == '#' { idx += tmp; } },
                    Some(ch) => { if *ch == '#' { idx += tmp; } },
                }

                tmp <<= 1;
            }
        }

        output.insert((*x, *y), image_ref[idx]);
    } 

    *void = next_void;
    output
}

fn process(image_ref: &[char], input: HashMap<(isize, isize), char>, time: usize) -> usize {
    let mut output: HashMap<(isize, isize), char> = input;
    let mut void: char = '.';

    for _ in 0..time {
        output = enhance(image_ref, output, &mut void);
    }

    output.values().fold(0, |base, ch| if *ch == '#' { base + 1 } else { base })
}
