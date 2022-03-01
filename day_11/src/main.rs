use std::fs;

#[derive(Clone, Copy)]
struct Octo {
    level: usize,
    flash: bool
}

impl Octo {
    fn new(ch: &char) -> Octo {
        Octo { level: ch.to_digit(10).unwrap() as usize, flash: false }
    }

    fn charge(&mut self, pos: &(usize, usize), stack: &mut [(usize, usize)], head: &mut usize) {
        self.level += 1;
        
        if self.level == 10 {
            self.level = 0;
            self.flash = true;
            stack[*head] = *pos;
            *head += 1;
        }
    }
}

fn main() {
    let mut octos: Vec<Vec<Octo>> = read_from_input("input");
    println!("Part 1: {}", part_1(&mut octos.clone()));
    println!("Part 2: {}", part_2(&mut octos));
}

fn read_from_input(filename: &str) -> Vec<Vec<Octo>> {
    fs::read_to_string(filename).expect("Read file error!")
        .split('\n')
        .filter(|row| !row.is_empty())
        .map(|row| {
            row.chars().map(|ch| Octo::new(&ch)).collect()
        })
        .collect()
}

fn part_1(octos: &mut Vec<Vec<Octo>>) -> usize {
    let mut count = 0;
    
    let mut stack: Vec<(usize, usize)> = vec![(0,0); 100];
    let mut head: usize = 0;

    for _ in 0..100 {
        progress(octos, &mut stack, &mut head);

        count += head;
        head = 0;
    }

    count
}

fn part_2(octos: &mut Vec<Vec<Octo>>) -> usize {
    let mut count = 0;
    
    let mut stack: Vec<(usize, usize)> = vec![(0,0); 100];
    let mut head: usize = 0;

    loop {
        progress(octos, &mut stack, &mut head);
        count += 1;

        if head == 100 {
            break;
        }
        head = 0;
    }

    count
}

fn progress(octos: &mut Vec<Vec<Octo>>, stack: &mut [(usize, usize)], head: &mut usize) {
    for (x, row) in octos.iter_mut().enumerate().take(10) {
        for (y, ele) in row.iter_mut().enumerate().take(10) {
            ele.flash = false;
            ele.charge(&(x, y), stack, head);
        }
    }

    let mut prev = 0;
    while *head != prev {
        charge_neighbor(octos, stack, head, &mut prev);
    }
}

fn charge_neighbor(octos: &mut Vec<Vec<Octo>>, stack: &mut [(usize, usize)], head: &mut usize, prev: &mut usize) {
    let tmp = *head;
    for idx in *prev..*head {
        let pos = stack[idx];
        let left = if pos.0 > 0 { pos.0 - 1 } else { 0 };
        let right = if pos.0 < 9 { pos.0 + 1 } else { 9 };
        let top = if pos.1 > 0 { pos.1 - 1 } else { 0 };
        let bot = if pos.1 < 9 { pos.1 + 1 } else { 9 };

        for (x, row) in octos.iter_mut().enumerate().take(right+1).skip(left) {
            for (y, ele) in row.iter_mut().enumerate().take(bot+1).skip(top) {
                if !ele.flash {
                    ele.charge(&(x, y), stack, head);
                }
            }
        }
    }
    *prev = tmp;
}
