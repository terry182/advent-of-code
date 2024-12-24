use std::cmp;
use std::io;
fn rotate(arr: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let m = arr.len();
    let n = arr[0].len();
    let mut next: Vec<Vec<char>> = vec![vec!['.'; m]; n];

    for i in 0..m {
        for j in 0..n {
            next[j][m - 1 - i] = arr[i][j];
        }
    }

    next
}
fn traverse(arr: &Vec<Vec<char>>) -> i32 {
    let p = "XMAS";
    let mut ans = 0;
    let m = arr.len();
    let n = arr[0].len();
    for i in 0..m {
        ans += String::from_iter(&arr[i]).matches(p).count();
    }

    let mut cross: Vec<char> = vec![];
    for cross_num in 0..(m + n) {
        let i = m.saturating_sub(cross_num);
        let j = cross_num.saturating_sub(m);
        for cnt in 0..cmp::min(m, n) {
            if i + cnt >= m || j + cnt >= n {
                break;
            }
            cross.push(arr[i + cnt][j + cnt]);
        }
        ans += String::from_iter(cross).matches(p).count();
        cross = vec![];
    }
    ans as i32
}

fn cross_traverse(arr: &Vec<Vec<char>>) -> i32 {
    // Search for
    // M.M
    // .A.
    // S.S
    let m = arr.len();
    let n = arr[0].len();
    let mut ans = 0;
    for i in 0..(m - 2) {
        for j in 0..(n - 2) {
            ans += (arr[i][j] == 'M'
                && arr[i][j + 2] == 'M'
                && arr[i + 1][j + 1] == 'A'
                && arr[i + 2][j] == 'S'
                && arr[i + 2][j + 2] == 'S') as i32;
        }
    }

    ans
}

fn main() {
    let stdin = io::stdin();
    let m: Vec<Vec<char>> = stdin
        .lines()
        .map(|x| x.unwrap().chars().collect())
        .collect();
    let mut arr: Vec<Vec<char>> = m.clone();

    let mut ans = 0;
    let mut cross_ans = 0;
    for _ in 0..4 {
        arr = rotate(&arr);
        ans += traverse(&arr);
        cross_ans += cross_traverse(&arr);
    }
    println!("{ans}");
    println!("{cross_ans}");
}
