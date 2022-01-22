// Part 1
// const DURATION: usize = 80;

// Part 2
const DURATION: usize = 256;

fn main() {
    // Part 1
    // let fishes: Vec<usize> = read_input("3,4,3,1,2");

    // Part 2
    let fishes: Vec<usize> = read_input("1,4,1,1,1,1,5,1,1,5,1,4,2,5,1,2,3,1,1,1,1,5,4,2,1,1,3,1,1,1,1,1,1,1,2,1,1,1,1,1,5,1,1,1,1,1,1,1,1,1,4,1,1,1,1,5,1,4,1,1,4,1,1,1,1,4,1,1,5,5,1,1,1,4,1,1,1,1,1,3,2,1,1,1,1,1,2,3,1,1,2,1,1,1,3,1,1,1,2,1,2,1,1,2,1,1,3,1,1,1,3,3,5,1,4,1,1,5,1,1,4,1,5,3,3,5,1,1,1,4,1,1,1,1,1,1,5,5,1,1,4,1,2,1,1,1,1,2,2,2,1,1,2,2,4,1,1,1,1,3,1,2,3,4,1,1,1,4,4,1,1,1,1,1,1,1,4,2,5,2,1,1,4,1,1,5,1,1,5,1,5,5,1,3,5,1,1,5,1,1,2,2,1,1,1,1,1,1,1,4,3,1,1,4,1,4,1,1,1,1,4,1,4,4,4,3,1,1,3,2,1,1,1,1,1,1,1,4,1,3,1,1,1,1,1,1,1,5,2,4,2,1,4,4,1,5,1,1,3,1,3,1,1,1,1,1,4,2,3,2,1,1,2,1,5,2,1,1,4,1,4,1,1,1,4,4,1,1,1,1,1,1,4,1,1,1,2,1,1,2");
    
    let mut birth: [usize; DURATION] = [0; DURATION];
    
    println!("{}", simulate(fishes, &mut birth));
}

fn read_input(input: &str) -> Vec<usize> {
    input.split(',').map(|num| num.parse().unwrap()).collect()
}

fn simulate(fishes: Vec<usize>, birth: &mut [usize]) -> usize {
    let mut fish_count: usize = fishes.len();

    fishes.iter().for_each(|fish| {
        birth[*fish] += 1;
    });

    for idx in 0..DURATION {
        fish_count += birth[idx];

        let child_idx = idx + 9;
        if child_idx < DURATION {
            birth[child_idx] += birth[idx];
        }

        let next_idx = idx + 7;
        if next_idx < DURATION {
            birth[next_idx] += birth[idx];
        }
    }

    fish_count
}
