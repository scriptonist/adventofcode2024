use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", p1(input));
    println!("{}", p2(input));
}

fn p1(input: &str) -> i32 {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let office: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let inc: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut inc_cycle = inc.iter().cycle();
    let mut current = inc_cycle.next().unwrap();
    let mut current_row: i32 = 0;
    let mut current_col: i32 = 0;
    for (row, row_chars) in office.iter().enumerate() {
        for (col, _) in row_chars.iter().enumerate() {
            if office[row][col] == '^' {
                (current_row, current_col) = (row as i32, col as i32);
                visited.insert((current_row, current_col));
                break;
            }
        }
    }
    loop {
        let (new_row, new_col) = (
            current_row as i32 + current.0,
            current_col as i32 + current.1,
        );
        if !is_valid_index(new_row, new_col, office.len() as i32, office.len() as i32) {
            return visited.len() as i32;
        }
        if office[new_row as usize][new_col as usize] == '#' {
            current = inc_cycle.next().unwrap()
        } else {
            // dbg!(new_row, new_col);
            visited.insert((new_row, new_col));
            current_row = new_row;
            current_col = new_col;
        }
    }
}

fn p2(input: &str) -> i32 {
    let office: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut current_row: i32 = 0;
    let mut current_col: i32 = 0;
    let mut count = 0;
    for (row, row_chars) in office.iter().enumerate() {
        for (col, _) in row_chars.iter().enumerate() {
            if office[row][col] == '^' {
                (current_row, current_col) = (row as i32, col as i32);
                break;
            }
        }
    }
    let mut office_cloned = office.clone();
    let (starting_row, starting_col) = (current_row, current_col);
    for (row, row_chars) in office.iter().enumerate() {
        for (col, original) in row_chars.iter().enumerate() {
            if (row, col) != (starting_row as usize, starting_col as usize) {
                office_cloned[row][col] = '#';
            }
            let (mut current_row, mut current_col) = (starting_row as i32, starting_col as i32);
            let inc: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
            let mut inc_cycle = inc.iter().cycle();
            let mut current = inc_cycle.next().unwrap();
            let mut visited: HashSet<((i32, i32), (i32, i32))> = HashSet::new();
            loop {
                let (new_row, new_col) = (
                    current_row as i32 + current.0,
                    current_col as i32 + current.1,
                );
                if !is_valid_index(new_row, new_col, office.len() as i32, office.len() as i32) {
                    break;
                }

                if office_cloned[new_row as usize][new_col as usize] == '#' {
                    if visited.contains(&((new_row, new_col), current.clone())) {
                        count += 1;
                        break
                    } else {
                        visited.insert(((new_row, new_col), current.clone()));
                    }
                    current = inc_cycle.next().unwrap()
                } else {
                    current_row = new_row;
                    current_col = new_col;
                }
            }
            office_cloned[row][col] = *original;
        }
    }

    count
}
fn is_valid_index(row: i32, column: i32, total_rows: i32, total_cols: i32) -> bool {
    row >= 0 && row < total_rows && column >= 0 && column < total_cols
}
