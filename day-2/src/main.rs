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
            for j in 1..=(len / 2) {
                if len % j == 0 {
                    let chunks: Vec<&[u8]> = s.as_bytes().chunks(j).collect();
                    if are_all_same(&chunks) {
                        all.push(i);
                        break;
                    }
                }
            }
        }
    }
    let sum: u64 = all.iter().sum();
    println!("{sum}");
}

// https://sts10.github.io/2019/06/06/is-all-equal-function.html
// https://mastodon.technology/@bugaevc/102226891784062955
fn are_all_same<T: PartialEq>(arr: &[T]) -> bool {
    arr.windows(2).all(|w| w[0] == w[1])
}
