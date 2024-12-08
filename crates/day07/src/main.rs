use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", p1(input));
    println!("{}", p2(input));
}

fn p1(input: &str) -> i64 {
    let mut total = 0;
    for line in input.lines() {
        let mut found: HashSet<i64> = HashSet::new();
        let (want, digits) = line.split_once(":").unwrap();
        let want = want.parse::<i64>().unwrap();
        let digits: Vec<i64> = digits
            .split(" ")
            .filter(|v| v.len() != 0)
            .map(|d| d.parse::<i64>().unwrap())
            .collect();
        results(digits[0], digits[1..].to_owned(), &mut found);
        if found.contains(&want) {
            total += want;
        }
    }
    total
}

fn p2(input: &str) -> i64 {
    let mut total = 0;
    for line in input.lines() {
        let mut found: HashSet<i64> = HashSet::new();
        let (want, digits) = line.split_once(":").unwrap();
        let want = want.parse::<i64>().unwrap();
        let digits: Vec<i64> = digits
            .split(" ")
            .filter(|v| v.len() != 0)
            .map(|d| d.parse::<i64>().unwrap())
            .collect();
        results_with_concat(digits[0], digits[1..].to_owned(), &mut found);
        if found.contains(&want) {
            total += want;
        }
    }
    total
}
fn results(current: i64, left: Vec<i64>, found: &mut HashSet<i64>) {
    if left.len() == 0 {
        found.insert(current);
        return;
    }
    results(current + left[0], left[1..].to_owned(), found);
    results(current * left[0], left[1..].to_owned(), found);
}

fn results_with_concat(current: i64, left: Vec<i64>, found: &mut HashSet<i64>) {
    if left.len() == 0 {
        found.insert(current);
        return;
    }
    crate::results_with_concat(current + left[0], left[1..].to_owned(), found);
    crate::results_with_concat(current * left[0], left[1..].to_owned(), found);
    crate::results_with_concat(
        (current.to_string() + left[0].to_string().as_str())
            .parse::<i64>()
            .unwrap(),
        left[1..].to_owned(),
        found,
    );
}
