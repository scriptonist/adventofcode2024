fn main() {
    let input = include_str!("../input.txt");
    println!("{}", p1(input));
    println!("{}", p2(input));
}

fn p1(input: &str) -> f64 {
    let mut next: Vec<String> = vec![];
    let mut total = 0.0;
    for line in input.lines() {
        if line.is_empty() {
            let game = GameSetup::new(&next);
            let (x, y) = game.solve();
            if x.fract() == 0.0 && y.fract() == 0.0 {
                let tokens = (x * 3.0)  + (y * 1.0);
                println!("{:?} {}", game, tokens);
                total += tokens;
            }
            next = vec![];
        } else {
            next.push(line.to_string());
        }
    }
    let game = GameSetup::new(&next);
    let (x, y) = game.solve();
    if x.fract() == 0.0 && y.fract() == 0.0 {
        let tokens = (x * 3.0)  + (y * 1.0);
        println!("{:?} {}", game, tokens);
        total += tokens;
    }
    
    total
}

fn p2(input: &str) -> f64 {
    let mut next: Vec<String> = vec![];
    let mut total = 0.0;
    for line in input.lines() {
        if line.is_empty() {
            let game = GameSetup::new_with_error(&next);
            let (x, y) = game.solve();
            if x.fract() == 0.0 && y.fract() == 0.0 {
                let tokens = (x * 3.0)  + (y * 1.0);
                println!("{:?} {}", game, tokens);
                total += tokens;
            }
            next = vec![];
        } else {
            next.push(line.to_string());
        }
    }
    let game = GameSetup::new_with_error(&next);
    let (x, y) = game.solve();
    if x.fract() == 0.0 && y.fract() == 0.0 {
        let tokens = (x * 3.0)  + (y * 1.0);
        println!("{:?} {}", game, tokens);
        total += tokens;
    }

    total
}


#[derive(Debug)]
struct GameSetup {
    a: (f64, f64),
    b: (f64, f64),
    prize: (f64, f64),
}

impl GameSetup {
    pub fn new(lines: &Vec<String>) -> Self {
        let a = lines[0].trim().split_once(",").unwrap();
        let a: (f64, f64) = (
            a.0.split_once('+').unwrap().1.parse().unwrap(),
            a.1.split_once("+").unwrap().1.parse().unwrap(),
        );
        let b = lines[1].trim().split_once(",").unwrap();
        let b: (f64, f64) = (
            b.0.split_once('+').unwrap().1.parse().unwrap(),
            b.1.split_once("+").unwrap().1.parse().unwrap(),
        );
        let prize = lines[2].split_once(",").unwrap();
        let prize: (f64, f64) = (prize.0.split_once('=').unwrap().1.parse().unwrap(), prize.1.split_once("=").unwrap().1.parse().unwrap());
        GameSetup {
            a,
            b,
            prize
        }
    }
    
    pub fn new_with_error(lines: &Vec<String>) -> Self {
        let a = lines[0].trim().split_once(",").unwrap();
        let a: (f64, f64) = (
            a.0.split_once('+').unwrap().1.parse().unwrap(),
            a.1.split_once("+").unwrap().1.parse().unwrap(),
        );
        let b = lines[1].trim().split_once(",").unwrap();
        let b: (f64, f64) = (
            b.0.split_once('+').unwrap().1.parse().unwrap(),
            b.1.split_once("+").unwrap().1.parse().unwrap(),
        );
        let prize = lines[2].split_once(",").unwrap();
        let prize: (f64, f64) = (prize.0.split_once('=').unwrap().1.parse().unwrap(), prize.1.split_once("=").unwrap().1.parse().unwrap());
        GameSetup {
            a,
            b,
           prize: (prize.0 + 10000000000000.0, prize.1 + 10000000000000.0)
        }
    }
    pub fn solve(&self) -> (f64, f64) {
        /*
        prize.0 = a.0 * k + b.0 * l
        prize.1 = a.1 * k + b.1 * l
        k =  [ prize.0 - b.0 * l ] / a.0 -- 1
        k = [ prize.1 - b.1 * l ] / a.1
        prize.0 - b.0 * l   prize.1 - b.1 * l
        ----------------- = ------------------
             a.0                  a.1

        prize.0 * a.1 - b.0 * l * a.1 = prize.1 * a.0 - b.1 * l * a.0
        b.1 * l * a.0 - b.0 * l * a.1 = prize1.1 * a.0 - prize.0 * a.1
        l ( b.1 * a.0 - b.0*a.1) = prize.1 * a.0 - prize.0 * a.1
        l = prize.1 * a.0 - prize.0 * a.1
            -----------------------------
            (b.1 * a.0 - b.0 * a.1)
         */
        let l = (self.prize.1 * self.a.0 - self.prize.0 * self.a.1)
            / (self.b.1 * self.a.0 - self.b.0 * self.a.1);
        let k = (self.prize.0 - self.b.0 * l) / self.a.0;

        (k, l)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve() {
        let game = GameSetup {
            a: (94f64, 34f64),
            b: (22f64, 67f64),
            prize: (8400f64, 5400f64),
        };
        let (x, y) = game.solve();
        assert_eq!(x, 80f64);
        assert_eq!(y, 40f64);

        let game = GameSetup {
            a: (17f64, 86f64),
            b: (84f64, 37f64),
            prize: (7870f64, 6450f64),
        };
        let (x, y) = game.solve();
        assert_eq!(x, 38f64);
        assert_eq!(y, 86f64);
    }
}
