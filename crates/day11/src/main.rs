use rayon::prelude::*;

fn main() {
    let input: Vec<i64> = include_str!("../input.txt")
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    println!("{}", blink_and_find(&input, 25));
    println!("{}", blink_and_find(&input, 75));
}

fn blink_and_find(input: &Vec<i64>, blinks: i64) -> u64 {
    let mut count: u64 = 0;
    let mut nums = input.clone();
    // println!("{}", blinks);
    for i in 0..blinks {
        // print!("{}.",i);
        nums = nums.par_iter().flat_map(|n| apply_rule(n)).collect();
        count = nums.len() as u64;
    }
    println!();
    count
}

fn apply_rule(n: &i64) -> Vec<i64> {
    if *n == 0 {
        return vec![1];
    }
    if n.to_string().len() % 2 == 0 {
        let n_s = n.to_string();
        let (left, right) = (
            n_s[..(n_s.len() / 2)].parse().unwrap(),
            n_s[n_s.len() / 2..].parse().unwrap(),
        );
        return vec![left, right];
    }
    vec![n * 2024]
}
