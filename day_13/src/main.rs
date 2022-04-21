use std::{fs, collections::HashSet};

enum Direct {
    X,
    Y
}

struct Fold {
    val: usize,
    dir: Direct
}

impl Fold {
    fn fold_points(&self, points: &HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
        let double_val: usize = self.val + self.val;

        points.iter().map(|point| {
            match self.dir {
                Direct::X => {
                    if point.0 < self.val {
                        return *point
                    }

                    (double_val - point.0, point.1)
                },
                Direct::Y => {
                    if point.1 < self.val {
                        return *point
                    }

                    (point.0, double_val - point.1)
                },
            }
        })
        .collect()
    }
}

fn main() {
    let (points, folds) = read_from_input("input");
    println!("Part 1: {}", part_1(&points, &folds));
    part_2(&points, &folds);
}

fn read_from_input(filename: &str) -> (HashSet<(usize, usize)>, Vec<Fold>) {
    let blocks: Vec<String> = fs::read_to_string(filename).expect("Read file error")
        .split("\n\n")
        .map(|row| row.to_string())
        .collect();

    let points: HashSet<(usize, usize)> = blocks[0].split('\n')
        .map(|row| {
            match row.split_once(',') {
                Some((str_x ,str_y)) => {
                    let x = str_x.parse::<usize>().unwrap();
                    let y = str_y.parse::<usize>().unwrap();

                    (x, y)
                },
                None => panic!(),
            }
        })
        .collect();

    let folds: Vec<Fold> = blocks[1].split('\n')
        .filter(|row| !row.is_empty())
        .map(|row| {
            let dir: Direct = match &row[11..12] {
                "x" => Direct::X,
                "y" => Direct::Y,
                _ => panic!()
            };

            let val: usize = row[13..].parse::<usize>().unwrap();

            Fold { val, dir }
        })
        .collect();

    (points, folds)
}

fn part_1(points: &HashSet<(usize, usize)>, folds: &[Fold]) -> usize {
    folds[0].fold_points(points).len()
}

fn part_2(points: &HashSet<(usize, usize)>, folds: &[Fold]) {
    let mut new_points: HashSet<(usize, usize)> = points.clone();
    
    folds.iter().for_each(|fold| {
        new_points = fold.fold_points(&new_points);
    });

    let mut max_x: usize = 0;
    let mut max_y: usize = 0;
    new_points.iter().for_each(|point| {
        if point.0 > max_x {
            max_x = point.0;
        }

        if point.1 > max_y {
            max_y = point.1;
        }
    });

    for y in 0..=max_y {
        for x in 0..=max_x {
            if new_points.contains(&(x ,y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
