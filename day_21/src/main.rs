use std::collections::HashMap;

fn main() {
    // test
    //let player_1: usize = 4;
    //let player_2: usize = 8;

    // input
    let player_1: usize = 6;
    let player_2: usize = 10;

    println!("Part 1: {}", part_1(player_1, player_2));
    println!("Part 2: {}", part_2(player_1, player_2));
}

fn part_1(mut player_1: usize, mut player_2: usize) -> usize {
    let mut dice: usize = 6;
    let mut roll: usize = 0;
    let mut score_1: usize = 0;
    let mut score_2: usize = 0;

    loop {
        player_1 += dice;
        player_1 = if player_1 > 10 { player_1 - 10 } else { player_1 };
        roll += 3;
        dice = if dice == 0 { 9 } else { dice - 1 };
        score_1 += player_1;
        if score_1 >= 1000 {
            return roll * score_2;
        }

        player_2 += dice;
        player_2 = if player_2 > 10 { player_2 - 10 } else { player_2 };
        roll += 3;
        dice = if dice == 0 { 9 } else { dice - 1 };
        score_2 += player_2;
        if score_2 >= 1000 {
            return roll * score_1;
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Game {
    space: (usize, usize),
    point: (usize, usize),
    turn: bool
}

impl Game {
    fn roll(&self, count: usize, map: &mut HashMap<Game, usize>, win_1: &mut usize, win_2: &mut usize) {
        for (dice, dice_cnt) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
            if self.turn {
                let new_space: usize = (self.space.0 + dice - 1) % 10 + 1;
                let new_point: usize = self.point.0 + new_space;
                if new_point > 20 {
                    *win_1 += count * dice_cnt;
                } else {
                    let entry = map.entry(Game { space: (new_space, self.space.1), point: (new_point, self.point.1), turn: false }).or_insert(0);
                    *entry += count * dice_cnt;
                }
            } else {
                let new_space: usize = (self.space.1 + dice - 1) % 10 + 1;
                let new_point: usize = self.point.1 + new_space;
                if new_point > 20 {
                    *win_2 += count * dice_cnt;
                } else {
                    let entry = map.entry(Game { space: (self.space.0, new_space), point: (self.point.0, new_point), turn: true }).or_insert(0);
                    *entry += count * dice_cnt;
                }
            }
        }
    }
}

fn part_2(player_1: usize, player_2: usize) -> usize {
    let mut win_1: usize = 0;
    let mut win_2: usize = 0;

    let mut map: HashMap<Game, usize> = HashMap::new();
    map.insert(Game { space: (player_1, player_2), point: (0, 0), turn: true }, 1);

    for loop_idx in 1..=21 {
        for point_1 in 0..(loop_idx-1) {
            update_map(&mut map, point_1, loop_idx-1, &mut win_1, &mut win_2);
        }
        for point_2 in 0..(loop_idx-1) {
            update_map(&mut map, loop_idx-1, point_2, &mut win_1, &mut win_2);
        }
        update_map(&mut map, loop_idx-1, loop_idx-1, &mut win_1, &mut win_2);
    }

    win_1.max(win_2)
}

fn update_map(map: &mut HashMap<Game, usize>, point_1: usize, point_2: usize, win_1: &mut usize, win_2: &mut usize) {
    for space_1 in 1..=10 {
        for space_2 in 1..=10 {
            let tmp_game: Game = Game { space: (space_1, space_2), point: (point_1, point_2), turn: true };
            match map.remove(&tmp_game) {
                None => {},
                Some(count) => tmp_game.roll(count, map, win_1, win_2),
            }
            let tmp_game: Game = Game { space: (space_1, space_2), point: (point_1, point_2), turn: false };
            match map.remove(&tmp_game) {
                None => {},
                Some(count) => tmp_game.roll(count, map, win_1, win_2),
            }
        }
    }
}
