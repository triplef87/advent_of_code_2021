use std::fs;

fn main() {
    let map: Vec<Vec<usize>> = read_from_file("input");
    println!("Part 1: {}", solve(&map));
    println!("Part 2: {}", part_2(&map));
}

fn read_from_file(filename: &str) -> Vec<Vec<usize>> {
    fs::read_to_string(filename).expect("Read file error")
        .split('\n')
        .filter(|row| !row.is_empty())
        .map(|row| {
            row.chars().map(|ch| ch.to_digit(10).unwrap() as usize).collect()
        })
        .collect()
}

fn solve(map: &[Vec<usize>]) -> usize {
    let height: usize = map.len();
    let width: usize = map[0].len();

    let mut visted: Vec<Vec<bool>> = vec![vec![false; width]; height];
    let mut risk: Vec<Vec<usize>> = vec![vec![usize::MAX; width]; height];

    visted[0][0] = true;
    risk[0][0] = 0;
    risk[1][0] = map[1][0];
    risk[0][1] = map[0][1];

    let mut stack: Vec<(usize, usize)> = vec![(0,0); height*width];
    stack[0] = (1, 0);
    stack[1] = (0, 1);
    let mut head: usize = 0;
    let mut end: usize = 2;
    while !visted[height-1][width-1] {
        let mut tmp: usize = usize::MAX;
        let mut tmp_idx: usize = 0;

        for (idx, val) in stack.iter().enumerate().take(end).skip(head) {
            let (tmp_x, tmp_y) = *val;
            if risk[tmp_x][tmp_y] < tmp {
                tmp = risk[tmp_x][tmp_y];
                tmp_idx = idx;
            }
        }

        // update head
        let (next_x, next_y) = stack[tmp_idx];
        stack[tmp_idx] = stack[head];
        head += 1;
        visted[next_x][next_y] = true;


        if next_x > 1 && !visted[next_x-1][next_y] {
            if risk[next_x-1][next_y] == usize::MAX {
                stack[end] = (next_x-1, next_y);
                end += 1;
            }
            risk[next_x-1][next_y] = risk[next_x-1][next_y].min(risk[next_x][next_y] + map[next_x-1][next_y]);
        }

        if next_y > 1 && !visted[next_x][next_y-1] {
            if risk[next_x][next_y-1] == usize::MAX {
                stack[end] = (next_x, next_y-1);
                end += 1;
            }
            risk[next_x][next_y-1] = risk[next_x][next_y-1].min(risk[next_x][next_y] + map[next_x][next_y-1]);
        }

        if next_x < height-1 && !visted[next_x+1][next_y] {
            if risk[next_x+1][next_y] == usize::MAX {
                stack[end] = (next_x+1, next_y);
                end += 1;
            }
            risk[next_x+1][next_y] = risk[next_x+1][next_y].min(risk[next_x][next_y] + map[next_x+1][next_y]);
        }

        if next_y < width-1 && !visted[next_x][next_y+1] {
            if risk[next_x][next_y+1] == usize::MAX {
                stack[end] = (next_x, next_y+1);
                end += 1;
            }
            risk[next_x][next_y+1] = risk[next_x][next_y+1].min(risk[next_x][next_y] + map[next_x][next_y+1]);
        }
    }

    risk[height-1][width-1]
}

fn part_2(map: &[Vec<usize>]) -> usize {
    let ori_height: usize = map.len();
    let ori_width: usize = map[0].len();

    let new_height: usize = ori_height * 5;
    let new_width: usize = ori_width * 5;
    let mut new_map: Vec<Vec<usize>> = vec![vec![0; new_width]; new_height];

    // Expand width dimension
    for offset in 0..5 {
        for height in 0..ori_height {
            for width in 0..ori_width {
                let tmp_width: usize = width + offset * ori_width;
                new_map[height][tmp_width] = map[height][width] + offset;

                if new_map[height][tmp_width] > 9 {
                    new_map[height][tmp_width] -= 9;
                }
            }
        }
    }

    // Expand height dimension
    for offset in 0..5 {
        for height in 0..ori_height {
            for width in 0..new_width {
                let tmp_height: usize = height + offset * ori_height;
                new_map[tmp_height][width] = new_map[height][width] + offset;

                if new_map[tmp_height][width] > 9 {
                    new_map[tmp_height][width] -= 9;
                }
            }
        }
    }

    solve(&new_map)
}
