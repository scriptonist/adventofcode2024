use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("../input.txt");
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    println!("{}", p1(&grid));
}

const INC_OPTS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Debug)]
struct Region {
    c: char,
    members: HashSet<(usize, usize)>,
}
fn p1(grid: &Vec<Vec<char>>) -> u32 {
    let mut regions: Vec<Region> = vec![];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    for (row, row_items) in grid.iter().enumerate() {
        for (col, item) in row_items.iter().enumerate() {
            if visited.contains(&(row, col)) {
                continue;
            }
            let members = find_all_neighbors(*item, (row, col), grid);
            for member in &members {
                visited.insert(member.clone());
            }
            regions.push(Region { c: *item, members });
        }
    }
    
    let mut total = 0;
    for region in regions {
        let mut perimeter: u32 = 0;
        let area = region.members.len() as u32;
        for position in region.members {
            for inc in INC_OPTS.iter() {
                let (new_row, new_col) = (position.0 as i32 + inc.0, position.1 as i32 + inc.1);
                if is_valid_index(new_row, new_col, grid.len(), grid.len()) {
                    if grid[new_row as usize][new_col as usize] != region.c {
                        perimeter += 1;
                        // print!("    {} ({} {}) ", region.c, new_row, new_col);
                    }
                } else if (new_row == -1) && (new_col >= 0 && new_col < grid.len() as i32) {
                    // is an top upper side
                    perimeter += 1;
                    // print!("    {} ({} {}) ", region.c, new_row, new_col);
                } else if new_row == grid.len() as i32
                    && (new_col >= 0 && new_col < grid[0].len() as i32)
                {
                    // lower side
                    perimeter += 1;
                    // print!("    {} ({} {}) ", region.c, new_row, new_col);
                } else if new_col == -1 && (new_row >= 0 && new_row < grid[0].len() as i32) {
                    // left
                    perimeter += 1;
                    // print!("    {} ({} {}) ", region.c, new_row, new_col);
                } else if new_col == grid[0].len() as i32
                    && (new_row >= 0 && new_row < grid.len() as i32)
                {
                    // right
                    perimeter += 1;
                    // print!("    {} ({} {}) ", region.c, new_row, new_col);
                }
            }
        }
        total += area * perimeter;
    }
    total
}

fn find_all_neighbors(
    c: char,
    (row, col): (usize, usize),
    grid: &Vec<Vec<char>>,
) -> HashSet<(usize, usize)> {
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    q.push_back((row, col));
    while !q.is_empty() {
        let (row, col) = q.pop_front().unwrap();
        visited.insert((row, col));
        for inc in INC_OPTS {
            let (new_row, new_col) = (inc.0 + row as i32, inc.1 + col as i32);
            if is_valid_index(new_row, new_col, grid.len(), grid.len()) {
                if grid[new_row as usize][new_col as usize] == c
                    && !visited.contains(&(new_row as usize, new_col as usize))
                {
                    q.push_back((new_row as usize, new_col as usize));
                }
            }
        }
    }
    visited
}

fn is_valid_index(row: i32, column: i32, total_rows: usize, total_cols: usize) -> bool {
    row >= 0 && row < total_rows as i32 && column >= 0 && column < total_cols as i32
}
