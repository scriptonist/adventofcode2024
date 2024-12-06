use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    day05(input);
}

fn day05(input: &str) {
    let mut total = 0;
    let mut lines_iter = input.lines();
    let mut after: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut before: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut incorrect_pages:Vec<Vec<i32>> = Vec::new();
    while let Some(line) = lines_iter.next() {
        if line.len() == 0 {
            break;
        }
        let (p1, p2) = line.split_once("|").unwrap();
        after
            .entry(p1.parse().unwrap())
            .and_modify(|v| v.push(p2.parse().unwrap()))
            .or_insert(vec![p2.parse().unwrap()]);
        before
            .entry(p2.parse().unwrap())
            .and_modify(|v| v.push(p1.parse().unwrap()))
            .or_insert(vec![p1.parse().unwrap()]);
    }
    for line in lines_iter {
        let mut pages_map: HashMap<i32, usize> = HashMap::new();
        let pages = line
            .split(",")
            .map(|n| n.parse().unwrap())
            .collect::<Vec<i32>>();
        for (idx, page) in pages.iter().enumerate() {
            pages_map.insert(*page, idx);
        }
        let mut is_ok = true;
        for (page, page_idx) in &pages_map {
            if let Some(afters) = after.get(&page) {
                for after in afters {
                    if let Some(after_idx) = pages_map.get(after) {
                        if !(page_idx < after_idx) {
                            is_ok = false;
                            break;
                        }
                    }
                }
                if !is_ok {
                    break;
                }
            }
        }
        if is_ok {
            let middle = pages[pages.len() / 2];
            total += middle;
            // dbg!(total, middle);
        } else {
           incorrect_pages.push(pages.clone()); 
        }
    }
    let mut incorrect_total = 0;
    for pages in incorrect_pages {
        for (idx, page) in pages.iter().enumerate() {
            let mut count_before = 0;
            let mut count_after = 0;
            println!("item: {}, before: {:?} after: {:?}", page, before.get(page), after.get(page));
            for (i, p) in pages.iter().enumerate() {
                if i == idx {
                   continue; 
                }
                if let Some(before_items) = before.get(page) {
                    if before_items.contains(p) {
                        count_before += 1;
                    } 
                }
                if let Some(after_items) = after.get(page) {
                    if after_items.contains(p) {
                        count_after += 1;
                    }
                }
            }
            if count_after == pages.len()/2 && count_before == pages.len()/2 {
                let middle = page;
                incorrect_total += middle;
                println!("middle: {} total: {}", middle, incorrect_total);
            }
        } 
    }
    println!("total: {}", total);
    println!("incorrect total: {}", incorrect_total);
}