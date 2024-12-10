use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    println!("{}", p1(&grid));
}

fn p1(grid: &Vec<Vec<char>>) -> i32 {
    let mut total = 0;
    for (row, row_items) in grid.iter().enumerate() {
        println!("{:?}", row_items);
    }
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    for (row, row_content) in grid.iter().enumerate() {
        for (col, c) in row_content.iter().enumerate() {
            if c.is_alphanumeric() {
                println!("{} {} : {}", row, col, c);
                for pair_row in row + 1..grid.len() {
                    for pair_col in 0..grid[pair_row].len() {
                        if grid[pair_row][pair_col] == *c {
                            println!(" {} {}", pair_row, pair_col);
                            if pair_col > col {
                                let (a_row, a_col) = (
                                    row as i32 - pair_row.abs_diff(row) as i32,
                                    col as i32 - pair_col.abs_diff(col) as i32,
                                );
                                if is_valid_index(a_row, a_col, grid.len(), grid.len()) {
                                    println!(
                                        "   {} {} : {}",
                                        a_row, a_col, grid[a_row as usize][a_col as usize]
                                    );
                                    visited.insert((a_row, a_col));
                                    total += 1
                                }
                                let (a_row, a_col) = (
                                    pair_row as i32 + pair_row.abs_diff(row) as i32,
                                    pair_col as i32 + pair_col.abs_diff(col) as i32,
                                );
                                if is_valid_index(a_row, a_col, grid.len(), grid.len()) {
                                    println!(
                                        "   {} {} : {}",
                                        a_row, a_col, grid[a_row as usize][a_col as usize]
                                    );
                                    visited.insert((a_row, a_col));
                                    total += 1
                                }
                            } else if pair_col < col {
                                let (a_row, a_col) = (
                                    row as i32 - pair_row.abs_diff(row) as i32,
                                    col as i32 + pair_col.abs_diff(col) as i32,
                                );
                                if is_valid_index(a_row, a_col, grid.len(), grid.len()) {
                                    println!(
                                        "   {} {} : {}",
                                        a_row, a_col, grid[a_row as usize][a_col as usize]
                                    );
                                    visited.insert((a_row, a_col));
                                    total += 1
                                }
                                let (a_row, a_col) = (
                                    pair_row as i32 + pair_row.abs_diff(row) as i32,
                                    pair_col as i32 - pair_col.abs_diff(col) as i32,
                                );
                                if is_valid_index(a_row, a_col, grid.len(), grid.len()) {
                                    println!(
                                        "   {} {} : {}",
                                        a_row, a_col, grid[a_row as usize][a_col as usize]
                                    );
                                    visited.insert((a_row, a_col));
                                    total += 1
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    visited.len() as i32
}

fn is_valid_index(row: i32, column: i32, total_rows: usize, total_cols: usize) -> bool {
    row >= 0 && row < total_rows as i32 && column >= 0 && column < total_cols as i32
}

