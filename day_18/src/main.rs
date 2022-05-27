use std::{fs ,iter};

enum Token {
    Open,
    Close,
    Val(u32)
}

fn main() {
    let content: Vec<String> = read_from_input("input");
    println!("Part 1: {}", part_1(&content));
    println!("Part 2: {}", part_2(&content));
}

fn read_from_input(filename: &str) -> Vec<String> {
    fs::read_to_string(filename).expect("Read file error!")
        .split('\n')
        .filter(|row| !row.is_empty())
        .map(|row| row.to_string())
        .collect()
}

fn part_1(content: &[String]) -> u32 {
    content.iter()
        .map(|row| parse_token(row))
        .reduce(add_token)
        .map(|token| cal(&token))
        .unwrap()
}

fn part_2(content: &[String]) -> u32 {
    let mut max: u32 = 0;

    for i in 0..(content.len()-1) {
        for j in i..(content.len()) {
            if content[i] == content[j] { continue; }

            let tmp_content_1: Vec<String> = vec![content[i].to_string(), content[j].to_string()];
            max = max.max(part_1(&tmp_content_1));
            
            let tmp_content_2: Vec<String> = vec![content[j].to_string(), content[i].to_string()];
            max = max.max(part_1(&tmp_content_2));
        }
    }

    max
}

fn parse_token(row: &str) -> Vec<Token> {
    row.chars().filter_map(|ch| match ch {
            '[' => Some(Token::Open),
            ']' => Some(Token::Close),
            _ => ch.to_digit(10).map(Token::Val),
        })
    .collect()
}

fn add_token(token: Vec<Token>, next: Vec<Token>) -> Vec<Token> {
    let mut new_token: Vec<Token> = iter::once(Token::Open)
        .chain(token.into_iter())
        .chain(next.into_iter())
        .chain(iter::once(Token::Close))
        .collect();

    loop {
        if explode(&mut new_token) { continue; }
        if split(&mut new_token) { continue; }
        break;
    }

    new_token
}

fn explode(new_token: &mut Vec<Token>) -> bool {
    let mut pairs: usize = 0;
    let mut idx: usize = 0;
    let mut stack: Vec<(usize, u32)> = Vec::new(); 

    while idx < new_token.len() {
        match new_token.get(idx).unwrap() {
            Token::Open => pairs += 1,
            Token::Val(digit) => { stack.push((idx, *digit)); },
            Token::Close => {
                if pairs < 5 {
                    pairs -= 1; 
                } else {
                    let (_, right): (_, u32) = stack.pop().unwrap();
                    let (_, left): (_, u32) = stack.pop().unwrap();

                    if let Some((prev_idx, prev_digit)) = stack.pop() {
                        let prev: &mut Token = new_token.get_mut(prev_idx).unwrap();
                        *prev = Token::Val(left + prev_digit); 
                    }

                    if let Some(digit) = new_token.iter_mut().skip(idx+1).find_map(|token| match token {
                        Token::Val(digit) => Some(digit),
                        _ => None
                    }) {
                        *digit += right;
                    }

                    idx -= 3;
                    new_token.remove(idx);
                    new_token.remove(idx);
                    new_token.remove(idx);
                    new_token[idx] = Token::Val(0);
                    return true;
                }
            },
        }

        idx += 1;
    }

    false
}

fn split(new_token: &mut Vec<Token>) -> bool {
    let mut idx: usize = 0;

    while idx < new_token.len() {
        if let Token::Val(digit) = new_token.get(idx).unwrap() {
            if *digit > 9 {
                let new_digit: u32 = *digit / 2;
                let reminder: u32 = *digit % 2;

                new_token.insert(idx, Token::Close);
                new_token.insert(idx, Token::Val(new_digit+reminder)); 
                new_token.insert(idx, Token::Val(new_digit)); 
                new_token.insert(idx, Token::Open);
                new_token.remove(idx + 4);

                return true;
            }
        }
        idx += 1;
    }

    false
}

fn cal(new_token: &[Token]) -> u32 {
    let mut stack: Vec<u32> = Vec::new();

    new_token.iter().for_each(|token| {
        match token {
            Token::Val(digit) => stack.push(*digit),
            Token::Close => {
                let right: u32 = stack.pop().unwrap();
                let left: u32 = stack.pop().unwrap();

                stack.push(3*left + 2*right);
            },
            Token::Open => {}
        }
    });

    stack[0]
}

