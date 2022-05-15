use std::collections::HashSet;

struct Target {
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize
}

fn main() {
    // Test
    //let target: Target = Target { x_min: 20, x_max: 30, y_min: -10, y_max: -5 };
    // Input
    let target: Target = Target { x_min: 169, x_max: 206, y_min: -108, y_max: -68 };

    println!("{}", part_1(&target));
    println!("{}", part_2(&target));
}

fn part_1(target: &Target) -> isize {
    target.y_min * (target.y_min + 1) / 2
}

fn part_2(target: &Target) -> usize {
    let mut velocity_set: HashSet<(isize, isize)> = HashSet::new();

    for x in 1..=(target.x_max) {
        for y in target.y_min..-target.y_min {
            if check_reach((x, y), target) { velocity_set.insert((x, y)); }
        }
    }

    velocity_set.len()
}

fn check_reach(mut velocity: (isize, isize), target: &Target) -> bool {
    let mut x: isize = 0;
    let mut y: isize = 0;

    while !(x > target.x_max || y < target.y_min) {
        x += velocity.0;
        y += velocity.1;
        
        if x >= target.x_min && x <= target.x_max && y >= target.y_min && y <= target.y_max {
            return true;
        }

        if velocity.0 > 0 {
            velocity.0 -= 1;
        }
        velocity.1 -= 1;
    }

    false
}
