use std::fs;

#[derive(Debug)]
struct Board {
    board: Vec<Vec<usize>>,
    marked: usize,
    bingo: bool,
    draw_cnt: [usize; 10]
}

impl Board {
    fn new(block: &String) -> Board {
        let board: Vec<Vec<usize>> = block.split('\n').map(|row| {
            row.split_whitespace().map(|num| num.parse::<usize>().unwrap()).collect()
        })
        .collect();

        Board { board, marked: 0, bingo: false, draw_cnt: [0; 10] }
    }

    fn draw(&mut self, num: &usize) {
        for x in 0..5 {
            for y in 0..5 {
                if self.board[x][y] == *num {
                    self.draw_cnt[x] = self.draw_cnt[x] + 1;
                    self.draw_cnt[5+y] = self.draw_cnt[5+y] + 1;

                    if self.draw_cnt[x] == 5 || self.draw_cnt[5+y] == 5 {
                        self.bingo = true;
                    }

                    self.marked = self.marked + *num;
                }
            }
        }
    }
}

fn main() {
    let mut blocks: Vec<String> = read_from_file("input");
    let input: Vec<usize> = blocks.remove(0).split(',')
        .map(|digit| digit.parse::<usize>().unwrap())
        .collect();
    
    let mut boards: Vec<Board> = blocks.iter().map(|board| { Board::new(board)}).collect();

    // println!("{}", part_1(&input, &mut boards));
    println!("{}", part_2(&input, &mut boards));
}

fn read_from_file(path: &str) -> Vec<String> {
    fs::read_to_string(path).expect("Read file error!")
        .split("\n\n")
        .filter(|row| !row.is_empty())
        .map(|row| row.to_string())
        .collect()
}

fn part_1(input: &Vec<usize>, boards: &mut Vec<Board>) -> usize {
    for num in input {
        for board in boards.iter_mut() {
            board.draw(num);

            if board.bingo {
                let mut un_mark: usize = 0;

                for x in 0..5 {
                    for y in 0..5 {                                            
                        un_mark = un_mark.wrapping_add(board.board[x][y]); 
                    }
                }
                un_mark = un_mark.wrapping_sub(board.marked);

                return un_mark.wrapping_mul(*num);
            }
        }
    }

    0
}

fn part_2(input: &Vec<usize>, boards: &mut Vec<Board>) -> usize {
    for num in input {
        boards.iter_mut().for_each(|board| board.draw(num));
        if boards.len() > 1 {
            boards.retain(|board| !board.bingo);
        } else if boards[0].bingo {
            let mut un_mark: usize = 0;
            for x in 0..5 {
                for y in 0..5 {
                    un_mark = un_mark.wrapping_add(boards[0].board[x][y]); 
                }
            }
            un_mark = un_mark.wrapping_sub(boards[0].marked);

            return un_mark.wrapping_mul(*num);
        }
    }

    0
}

#[test]
fn test_1() {
    let mut blocks: Vec<String> = read_from_file("test");
    let input: Vec<usize> = blocks.remove(0).split(',')
        .map(|digit| digit.parse::<usize>().unwrap())
        .collect();
    
    let mut boards: Vec<Board> = blocks.iter().map(|board| { Board::new(board)}).collect();

    
    assert_eq!(4512, part_1(&input, &mut boards));
}

#[test]
fn test_2() {
    let mut blocks: Vec<String> = read_from_file("test");
    let input: Vec<usize> = blocks.remove(0).split(',')
        .map(|digit| digit.parse::<usize>().unwrap())
        .collect();
    
    let mut boards: Vec<Board> = blocks.iter().map(|board| { Board::new(board)}).collect();

    
    assert_eq!(1924, part_2(&input, &mut boards));
}
