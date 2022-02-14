use std::fs;

fn main() {
    let height_map: Vec<Vec<usize>> = read_from_file("input");

    let (risk, low_points) = part_1(&height_map); 
    println!("Part 1: {risk}");
    println!("Part 2: {}", part_2(&height_map, &low_points));
}

fn read_from_file(file_path: &str) -> Vec<Vec<usize>> {
    fs::read_to_string(file_path).expect("Read file fail!")
        .split('\n')
        .filter(|row| !row.is_empty())
        .map(|row| {
            row.chars().map(|ch| ch.to_digit(10).unwrap() as usize).collect()
        })
        .collect()
}

fn part_1(height_map: &[Vec<usize>]) -> (usize, Vec<(usize, usize)>) {
    let mut risk: usize = 0;

    let width: usize = height_map[0].len() - 1;
    let height: usize = height_map.len() - 1;

    let mut low_points: Vec<(usize, usize)> = Vec::new();
    for x in 0..height+1 {
        for y in 0..width+1 {
            if !(x == 0 || (height_map[x][y] < height_map[x-1][y])) {
                continue;
            }

            if !(y == 0 || (height_map[x][y] < height_map[x][y-1])) {
                continue;
            }

            if !(x == height || (height_map[x][y] < height_map[x+1][y])) {
                continue;
            }
            
            if !(y == width || (height_map[x][y] < height_map[x][y+1])) {
                continue;
            }

            risk += height_map[x][y] + 1;
            low_points.push((x, y));
        }
    }

    (risk, low_points)
}

fn part_2(height_map: &[Vec<usize>], low_points: &[(usize, usize)]) -> usize {
    let height: usize = height_map.len();
    let width: usize = height_map[0].len();

    let mut color_map: Vec<Vec<usize>> = vec![vec![0; width]; height];
    let mut basins: Vec<usize> = vec![0; low_points.len()+1];
    let mut stack: Vec<(usize, usize)> = vec![(0,0); width*height];

    for (idx, point) in low_points.iter().enumerate() {
        color_map[point.0][point.1] = idx + 1;
        basins[idx+1] = 1;
        stack[idx] = *point;
    }
    
    let mut head = 0;
    let mut end = low_points.len();

    let tmp_height = height - 1;
    let tmp_width = width - 1;
    while head < end {
        let (x, y) = stack[head];
        let color = color_map[x][y];

        if !(x == 0 || (height_map[x-1][y] == 9) || (color_map[x-1][y] != 0)) {
            color_map[x-1][y] = color;
            stack[end] = (x-1, y);
            end += 1;
            basins[color] += 1;
        }

        if !(y == 0 || (height_map[x][y-1] == 9) || (color_map[x][y-1] != 0)) {
            color_map[x][y-1] = color;
            stack[end] = (x, y-1);
            end += 1;
            basins[color] += 1;
        }

        if !(x == tmp_height || (height_map[x+1][y] == 9) || (color_map[x+1][y] != 0)) {
            color_map[x+1][y] = color;
            stack[end] = (x+1, y);
            end += 1;
            basins[color] += 1;
        }
        
        if !(y == tmp_width || (height_map[x][y+1] == 9) || (color_map[x][y+1] != 0)) {
            color_map[x][y+1] = color;
            stack[end] = (x, y+1);
            end += 1;
            basins[color] += 1;
        }

        head += 1;
    }

    basins.sort_by(|a, b| b.cmp(a));
    
    basins[0] * basins[1] * basins[2]
}

#[test]
fn test() {
    let height_map: Vec<Vec<usize>> = read_from_file("test");

    let (risk, low_points) = part_1(&height_map); 
    
    assert_eq!(15, risk);
    assert_eq!(1134, part_2(&height_map, &low_points));
}
