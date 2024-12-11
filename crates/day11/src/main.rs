fn main() {
    let input: Vec<i64> = include_str!("../input.txt")
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    println!("{}", blink_and_find(&input, 25));
    println!("{}", blink_and_find(&input, 75));
}

fn blink_and_find(input: &Vec<i64>, blinks: i32) -> u64 {
    let mut nums = input.clone();
    for _ in 0..blinks {
        let nums_iter = nums.clone();
        let mut actual_idx = 0;
        for n in nums_iter.iter() {
            let new_n = apply_rule(n);
            nums.splice(actual_idx..actual_idx+1, new_n.clone());
            actual_idx += new_n.len();
            // println!("{:?}", nums);
        }
    }
    nums.len() as u64
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
