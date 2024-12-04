fn main() {
    let input = include_str!("../input.txt");

    let input_vec: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    println!("{}", p1(&input_vec));
    println!("{}", p2(&input_vec));
}

fn p1(input: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;

    let directions: [[(i32, i32); 3]; 8] = [
        [(1, 0), (2, 0), (3, 0)],
        [(-1, 0), (-2, 0), (-3, 0)],
        [(0, 1), (0, 2), (0, 3)],
        [(0, -1), (0, -2), (0, -3)],
        [(1, 1), (2, 2), (3, 3)],
        [(-1, 1), (-2, 2), (-3, 3)],
        [(1, -1), (2, -2), (3, -3)],
        [(-1, -1), (-2, -2), (-3, -3)],
    ];

    for (row, line) in input.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            for direction in &directions {
                let mut s: Vec<char> = vec![*c];
                for (row_delta, col_delta) in direction {
                    let (new_row, new_col) = (row as i32 + row_delta, col as i32 + col_delta);
                    if is_valid_index(new_row, new_col, line.len() as i32, line.len() as i32) {
                        s.push(input[new_row as usize][new_col as usize]);
                    }
                }
                if s == vec!['X', 'M', 'A', 'S'] {
                    count += 1;
                }
            }
        }
    }
    count
}

fn p2(input: &Vec<Vec<char>>) -> u32 {
    let mut count = 0;
    /*
    M.S
    .A.
    M.S                                 
     */
    //                                  S       S        M         M
    let directions: [(i32, i32); 4] = [(1, 1), (-1, 1), (1, -1), (-1, -1)];

    for (row, line) in input.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            if *c != 'A' {
                continue;
            }
            let mut s: Vec<char> = vec![*c];
            for (row_delta, col_delta) in directions {
                let (new_row, new_col) = (row as i32 + row_delta, col as i32 + col_delta);
                if is_valid_index(new_row, new_col, line.len() as i32, line.len() as i32) {
                    // println!("{}, {}", new_row, new_col);
                    s.push(input[new_row as usize][new_col as usize]);
                }
            }
            // println!("{} {} {}", row, col, &s.iter().collect::<String>());
            if [
                vec!['A','S', 'S', 'M', 'M'],
                vec!['A','S', 'M', 'S', 'M'],
                vec!['A','M', 'S', 'M', 'S'],
                vec!['A', 'M', 'M', 'S', 'S']
            ].contains(&s) {
               count += 1; 
            }
        }
    }
    count
}
fn is_valid_index(row: i32, column: i32, total_rows: i32, total_cols: i32) -> bool {
    row >= 0 && row < total_rows && column >= 0 && column < total_cols
}
