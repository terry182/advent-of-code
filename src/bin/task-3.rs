use regex::Regex;
use std::io;

fn main() {
    let stdin = io::stdin();
    let mul = Regex::new(r"mul\((\d*),(\d*)\)|do\(\)|don't\(\)").unwrap();
    let mut ans = 0;
    let mut multiplier = 1;
    for line in stdin.lines() {
        let s = line.unwrap();
        for capture in mul.captures_iter(s.as_str()) {
            match capture.get(1) {
                None => {
                    match capture.get(0).unwrap().as_str().len() {
                        4 => multiplier = 1, // len("do()") == 4
                        7 => multiplier = 0, // len("don't()") == 7
                        _ => {}
                    }
                }
                Some(_a) => {
                    let a: i32 = _a.as_str().parse().unwrap();
                    let b: i32 = capture.get(2).unwrap().as_str().parse().unwrap();
                    ans += a * b * multiplier;
                }
            }
        }
    }
    println!("{ans}");
}
