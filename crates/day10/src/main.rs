use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("../input.txt");
    let trail: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    println!("{}", p1(&trail));
    println!("{}", p2(&trail));
}

fn p1(trail: &Vec<Vec<char>>) -> i32 {
    let mut score = 0;
    for (row, row_items) in trail.iter().enumerate() {
        for (col, _) in row_items.iter().enumerate() {
            let (s, _) = count_trails(row as i32, col as i32, trail);
            score += s;
        }
    }
    score
}

fn p2(trail: &Vec<Vec<char>>) -> i32 {
    let mut paths = 0;
    for (row, row_items) in trail.iter().enumerate() {
        for (col, _) in row_items.iter().enumerate() {
            let (_, p) = count_trails(row as i32, col as i32, trail);
            paths += p;
        }
    }
    paths
}
fn count_trails(start_row: i32, start_col: i32, trail_map: &Vec<Vec<char>>) -> (i32, i32) {
    if let Some(current_val) = trail_map[start_row as usize][start_col as usize].to_digit(10) {
        if current_val != 0 {
            return (0, 0);
        }
    } else {
        return (0, 0);
    }
    let mut paths = 0;
    let mut q: VecDeque<(i32, i32)> = VecDeque::new();
    q.push_back((start_row, start_col));
    let mut trail_finishes:HashSet<(i32, i32)> = HashSet::new();
    let next_pos_inc: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    while let Some((row, col)) = q.pop_front() {
        let current_val: u32 = trail_map[row as usize][col as usize].to_digit(10).unwrap();
        if current_val == 9 {
            trail_finishes.insert((row, col));
            paths += 1;
            continue
        }
        
        let next_positions = next_pos_inc
            .iter()
            .map(|(inc_row, inc_col)| (row + inc_row, col + inc_col))
            .collect::<Vec<(i32, i32)>>();
        for (r, c) in next_positions {
            if is_valid_index(r, c, trail_map.len(), trail_map.len()) {
                if let Some(v) = trail_map[r as usize][c as usize].to_digit(10) {
                    if v > current_val && (v - current_val) == 1 {
                        q.push_back((r, c));
                    }
                }
            }
        }
    }
    (trail_finishes.len() as i32, paths)
}

fn is_valid_index(row: i32, column: i32, total_rows: usize, total_cols: usize) -> bool {
    row >= 0 && row < total_rows as i32 && column >= 0 && column < total_cols as i32
}
