use std::fs;

#[derive(Clone)]
struct Cube {
    x: isize,
    y: isize,
    z: isize
}

#[derive(Clone)]
struct Step {
    turn: bool,
    max: Cube,
    min: Cube
}

impl Step {
    fn parse(row: &str) -> Step {
        let split: Vec<&str> = row.split_whitespace().collect();
        let turn: bool = split[0] == "on"; 

        let mut min: Cube = Cube { x: 0, y:0 , z:0 };
        let mut max: Cube = Cube { x: 0, y:0 , z:0 };

        split[1].split(',').enumerate().for_each(|(idx, ele)| {
            let (head, end): (&str, &str) = ele[2..].split_once("..").unwrap();
            match idx {
                0 => {
                    min.x = head.parse::<isize>().unwrap(); 
                    max.x = end.parse::<isize>().unwrap(); 
                },
                1 => {
                    min.y = head.parse::<isize>().unwrap(); 
                    max.y = end.parse::<isize>().unwrap(); 
                },
                2 => {
                    min.z = head.parse::<isize>().unwrap(); 
                    max.z = end.parse::<isize>().unwrap(); 
                },
                _ => {}
            }
        });

        Step { turn, min, max }
    }
    fn intersect(&self, step: &Step) -> Option<Step> {
        let min_x: isize = self.min.x.max(step.min.x);
        let max_x: isize = self.max.x.min(step.max.x);
        let min_y: isize = self.min.y.max(step.min.y);
        let max_y: isize = self.max.y.min(step.max.y);
        let min_z: isize = self.min.z.max(step.min.z);
        let max_z: isize = self.max.z.min(step.max.z);

        if min_x > max_x || min_y > max_y || min_z > max_z {
            None
        } else {
            Some(Step {
                    turn: !self.turn,
                    min: Cube { x: min_x, y: min_y, z: min_z },
                    max: Cube { x: max_x, y: max_y, z: max_z } })
        }
    }
}

fn main() {
    let steps: Vec<Step> = read_from_input("input");
    println!("Part 1: {}", part_1(&steps));
    println!("Part 2: {}", part_2(&steps));
}

fn read_from_input(filename: &str) -> Vec<Step> {
    fs::read_to_string(filename).expect("Read file error!")
        .split('\n')
        .filter(|row| !row.is_empty())
        .map(Step::parse)
        .collect()
}

fn part_1(steps: &[Step]) -> usize {
    let mut limit: Step = Step { turn: true, min: Cube { x: -50, y: -50, z: -50 }, max: Cube { x: 50, y: 50, z: 50 } };
    let new_steps: Vec<Step> = steps.iter()
        .filter_map(|step| {
            limit.turn = !step.turn;
            limit.intersect(step)
        })
        .collect();

    solve(&new_steps)
}

fn part_2(steps: &[Step]) -> usize {
    solve(steps)
}

fn solve(steps: &[Step]) -> usize {
    let mut new_steps: Vec<Step> = vec![];

    steps.iter().for_each(|step| {
        let len: usize = new_steps.len();

        if step.turn {
            new_steps.push(step.clone());
        }

        for idx in 0..len {
            match new_steps[idx].intersect(step) {
                None => {},
                Some(inter) => { new_steps.push(inter); }
            }
        }
    });
    
    new_steps.iter().fold(0, |base, step| {
        if step.turn {
            base + (step.max.x.abs_diff(step.min.x) + 1) * (step.max.y.abs_diff(step.min.y) + 1) * (step.max.z.abs_diff(step.min.z) + 1)
        } else {
            base - (step.max.x.abs_diff(step.min.x) + 1) * (step.max.y.abs_diff(step.min.y) + 1) * (step.max.z.abs_diff(step.min.z) + 1)
        }
    })
}
