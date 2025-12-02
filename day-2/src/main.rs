use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let ranges = INPUT.trim_ascii().split(',').map(|x| {
        let (a, b) = x.split_once('-').unwrap();
        let a = u64::from_str(a).unwrap();
        let b = u64::from_str(b).unwrap();
        a..=b
    });
    let mut all = vec![];
    for range in ranges {
        for i in range {
            let s = i.to_string();
            let len = s.len();
            if len % 2 == 0 {
                let (a, b) = s.split_at(len / 2);
                if a == b {
                    all.push(i);
                }
            }
        }
    }
    let sum: u64 = all.iter().sum();
    println!("{sum}");
}
