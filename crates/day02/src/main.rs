fn main() {
    let input = include_str!("../input.txt");
    println!("{}", p1(input));
    println!("{}", p2(input));
}

fn p1(input: &str) -> i32 {
    let mut count = 0;
    for line in input.lines() {
        let report: Vec<i32> = line
            .split_whitespace()
            .map(|v| v.parse::<i32>().unwrap())
            .collect();
        if is_valid_report(&report) {
            count += 1;
        }
    }
    count
}

fn p2(input: &str) -> i32 {
    let mut count = 0;
    for line in input.lines() {
        let report: Vec<i32> = line
            .split_whitespace()
            .map(|v| v.parse::<i32>().unwrap())
            .collect();
        if is_valid_report(&report) {
            count += 1;
        } else {
            for idx in 0..report.len() {
                let mut new_report = report.to_vec();
                new_report.remove(idx);
                if is_valid_report(&new_report) {
                    count += 1;
                    break;
                }
            }
        }
    }
    count
}
enum Ordering {
    Greater,
    Lesser,
}
fn is_valid_report(report: &Vec<i32>) -> bool {
    let mut report_iter = report.iter();
    let first = report_iter.next().unwrap();
    let second = report.iter().nth(1).unwrap();
    let ordering: Ordering;
    if second > first {
        ordering = Ordering::Greater
    } else if second < first {
        ordering = Ordering::Lesser
    } else {
        return false;
    }
    let mut previous = first;
    for next in report_iter {
        if previous == next {
            return false;
        }

        let mut diff: i32 = 0;
        match ordering {
            Ordering::Greater => {
                if next > previous {
                    diff = next - previous;
                }
            }
            Ordering::Lesser => {
                if next < previous {
                    diff = previous - next;
                }
            }
        }
        if diff < 1 {
            return false;
        }
        if diff > 3 {
            return false;
        }
        previous = next
    }
    true
}
