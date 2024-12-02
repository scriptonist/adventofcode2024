use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", p1(input));
    println!("{}", p2(input));
}


fn p1(input: &str) -> u32 {
    let mut left_heap = BinaryHeap::new();
    let mut right_heap = BinaryHeap::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let (left, right) = (parts[0].parse::<i32>().unwrap(), parts[1].parse::<i32>().unwrap());
        dbg!(left, right);
        left_heap.push(Reverse(left));
        right_heap.push(Reverse(right));
    }

    let mut total = 0;
    while !left_heap.is_empty() {
        let left = left_heap.pop().unwrap().0;
        let right = right_heap.pop().unwrap().0;
        let distance = left.abs_diff(right);
        println!();
        dbg!(left, right, distance);
        total += distance;
    }
    total
}

fn p2(input: &str) -> i32 {
    let mut sim_score: i32 = 0;
    let mut count_map:HashMap<i32, i32> = HashMap::new();
    let mut left_items: Vec<i32> = vec![];
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let (left, right) = (parts[0].parse::<i32>().unwrap(), parts[1].parse::<i32>().unwrap());
        left_items.push(left);
        count_map.entry(right).and_modify(|count| *count += 1).or_insert(1);
    }

    for item in left_items {
        sim_score += item * count_map.get(&item).unwrap_or(&0);
    }

    sim_score
}
