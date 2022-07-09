use std::{fs, collections::{HashMap, HashSet}};

static ROTATE_ARR: [[Point; 3]; 24] = [
    // x, y, z => x, y, z
    [Point { x: 1, y: 0, z: 0 }, Point { x: 0, y: 1, z: 0 }, Point { x: 0, y: 0, z: 1 }],
    [Point { x: 1, y: 0, z: 0 }, Point { x: 0, y: -1, z: 0 }, Point { x: 0, y: 0, z: -1 }],
    [Point { x: -1, y: 0, z: 0 }, Point { x: 0, y: 1, z: 0 }, Point { x: 0, y: 0, z: -1 }],
    [Point { x: -1, y: 0, z: 0 }, Point { x: 0, y: -1, z: 0 }, Point { x: 0, y: 0, z: 1 }],

    // x, y, z => x, z, y
    [Point { x: 1, y: 0, z: 0 }, Point { x: 0, y: 0, z: 1 }, Point { x: 0, y: -1, z: 0 }],
    [Point { x: 1, y: 0, z: 0 }, Point { x: 0, y: 0, z: -1 }, Point { x: 0, y: 1, z: 0 }],
    [Point { x: -1, y: 0, z: 0 }, Point { x: 0, y: 0, z: -1 }, Point { x: 0, y: -1, z: 0 }],
    [Point { x: -1, y: 0, z: 0 }, Point { x: 0, y: 0, z: 1 }, Point { x: 0, y: 1, z: 0 }],

    // x, y, z => y, x, z
    [Point { x: 0, y: 1, z: 0 }, Point { x: 1, y: 0, z: 0 }, Point { x: 0, y: 0, z: -1 }],
    [Point { x: 0, y: 1, z: 0 }, Point { x: -1, y: 0, z: 0 }, Point { x: 0, y: 0, z: 1 }],
    [Point { x: 0, y: -1, z: 0 }, Point { x: 1, y: 0, z: 0 }, Point { x: 0, y: 0, z: 1 }],
    [Point { x: 0, y: -1, z: 0 }, Point { x: -1, y: 0, z: 0 }, Point { x: 0, y: 0, z: -1 }],

    // x, y, z => y, z, x
    [Point { x: 0, y: 1, z: 0 }, Point { x: 0, y: 0, z: -1 }, Point { x: -1, y: 0, z: 0 }],
    [Point { x: 0, y: 1, z: 0 }, Point { x: 0, y: 0, z: 1 }, Point { x: 1, y: 0, z: 0 }],
    [Point { x: 0, y: -1, z: 0 }, Point { x: 0, y: 0, z: 1 }, Point { x: -1, y: 0, z: 0 }],
    [Point { x: 0, y: -1, z: 0 }, Point { x: 0, y: 0, z: -1 }, Point { x: 1, y: 0, z: 0 }],

    // x, y, z => z, x, y
    [Point { x: 0, y: 0, z: 1 }, Point { x: 1, y: 0, z: 0 }, Point { x: 0, y: 1, z: 0 }],
    [Point { x: 0, y: 0, z: 1 }, Point { x: -1, y: 0, z: 0 }, Point { x: 0, y: -1, z: 0 }],
    [Point { x: 0, y: 0, z: -1 }, Point { x: -1, y: 0, z: 0 }, Point { x: 0, y: 1, z: 0 }],
    [Point { x: 0, y: 0, z: -1 }, Point { x: 1, y: 0, z: 0 }, Point { x: 0, y: -1, z: 0 }],

    // x, y, z => z, y, x
    [Point { x: 0, y: 0, z: 1 }, Point { x: 0, y: 1, z: 0 }, Point { x: -1, y: 0, z: 0 }],
    [Point { x: 0, y: 0, z: 1 }, Point { x: 0, y: -1, z: 0 }, Point { x: 1, y: 0, z: 0 }],
    [Point { x: 0, y: 0, z: -1 }, Point { x: 0, y: -1, z: 0 }, Point { x: -1, y: 0, z: 0 }],
    [Point { x: 0, y: 0, z: -1 }, Point { x: 0, y: 1, z: 0 }, Point { x: 1, y: 0, z: 0 }]
];

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
    z: isize
}

impl Point {
    fn rotate(&self, rotate_id: usize) -> Point {
        Point {
            x: self.x * ROTATE_ARR[rotate_id][0].x + self.y * ROTATE_ARR[rotate_id][1].x + self.z * ROTATE_ARR[rotate_id][2].x,
            y: self.x * ROTATE_ARR[rotate_id][0].y + self.y * ROTATE_ARR[rotate_id][1].y + self.z * ROTATE_ARR[rotate_id][2].y,
            z: self.x * ROTATE_ARR[rotate_id][0].z + self.y * ROTATE_ARR[rotate_id][1].z + self.z * ROTATE_ARR[rotate_id][2].z,
        }
    }

    fn add(&self, other_point: &Point) -> Point {
        Point { x: self.x + other_point.x, y: self.y + other_point.y, z: self.z + other_point.z }
    }

    fn sub(&self, other_point: &Point) -> Point {
        Point { x: self.x - other_point.x, y: self.y - other_point.y, z: self.z - other_point.z }
    }

    fn distance(&self, other_point: &Point) -> usize {
        self.x.abs_diff(other_point.x) + self.y.abs_diff(other_point.y) + self.z.abs_diff(other_point.z)
    }
}

#[derive(Clone)]
struct Scanner {
    point: Point,
    beacon: Vec<Point>
}

impl Scanner {
    fn rotate(&self, rotate_id: usize) -> Scanner {
        let mut new_beacon: Vec<Point> = Vec::new();
        self.beacon.iter().for_each(|point| new_beacon.push(point.rotate(rotate_id)));

        Scanner { point: self.point, beacon: new_beacon }
    }
}

fn main() {
    let mut scanners: Vec<Scanner> = read_from_input("input");
    println!("Part 1: {}", part_1(&mut scanners));
    println!("Part 2: {}", part_2(&scanners));
}

fn read_from_input(filename: &str) -> Vec<Scanner> {
    fs::read_to_string(filename).expect("Read file error!")
        .split("\n\n")
        .map(|data| {
            let beacon: Vec<Point> = data.split('\n')
                .filter(|row| !row.is_empty())
                .skip(1)
                .map(|row| {
                    let point: Vec<&str> = row.split(',').collect(); 

                    Point { 
                        x: point[0].parse::<isize>().unwrap(),
                        y: point[1].parse::<isize>().unwrap(),
                        z: point[2].parse::<isize>().unwrap()
                    }
                })
                .collect();

            Scanner { point: Point { x: 0, y: 0, z: 0 }, beacon }
        })
        .collect()
}

fn part_1(scanners: &mut Vec<Scanner>) -> usize {
    let mut stack: Vec<usize> = (0..scanners.len()).into_iter().collect();
    let mut check: Vec<usize> = vec![stack.remove(0)];

    while !check.is_empty() {
        let base_idx: usize = check.pop().unwrap();
        let base_scanner: Scanner = scanners.get(base_idx).unwrap().clone();
        let mut new_check: Vec<usize> = Vec::new();

        stack.retain(|&stack_idx| {
            let scanner: &Scanner = scanners.get(stack_idx).unwrap(); 
            
            for rotate_idx in 0..24 {
                let mut tmp_scanner: Scanner = scanner.rotate(rotate_idx);
                let mut diff_map: HashMap<Point, usize> = HashMap::new();

                for base_point in base_scanner.beacon.iter() {
                    for rotate_point in tmp_scanner.beacon.iter() {
                        let diff: Point = base_point.sub(rotate_point);
                        let entry = diff_map.entry(diff).or_insert(0);
                        *entry += 1;
                    }
                }

                for (diff, count) in diff_map {
                    if count >= 12 {
                        tmp_scanner.point = base_scanner.point.add(&diff);
                        scanners[stack_idx] = tmp_scanner;
                        new_check.push(stack_idx);

                        return false;
                    }
                }
            }
            
            true
        });
       
        check.append(&mut new_check);
    }

    let mut point_set: HashSet<Point> = HashSet::new();
    for scanner in scanners {
        for point in &scanner.beacon {
            point_set.insert(scanner.point.add(point));
        }
    }

    point_set.len()
}

fn part_2(scanners: &Vec<Scanner>) -> usize {
    let mut max: usize = 0;
    let len: usize = scanners.len();

    for base in 0..len-1 {
        for cmp in base+1..len {
            max = max.max(scanners[base].point.distance(&scanners[cmp].point));
        }
    }

    max
}
