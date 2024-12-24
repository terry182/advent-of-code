use std::io;

fn main() {
    let stdin = io::stdin();
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];
    for line in stdin.lines() {
        let s = line.unwrap();
        let mut num = s.split_whitespace();
        left.push(num.next().unwrap().parse().unwrap());
        right.push(num.next().unwrap().parse().unwrap());
    }
    left.sort();
    right.sort();

    // Part 1
    let mut ans = 0;
    for i in 0..left.len() {
        ans += left[i].abs_diff(right[i]);
    }

    // Part 2
    ans = 0;
    let mut lcount = 0; // appeared in right hand side
    let mut rcount = 0;
    let mut ri = 0;
    let mut li = 0;

    while li < left.len() && ri < right.len() {
        if left[li] < right[ri] {
            li += 1;
        } else if left[li] == right[ri] {
            let num = left[li];
            while li < left.len() && left[li] == num {
                lcount += 1;
                li += 1;
            }
            while ri < right.len() && right[ri] == num {
                rcount += 1;
                ri += 1;
            }
            ans += num * lcount * rcount;
            lcount = 0;
            rcount = 0;
        } else {
            ri += 1;
        }
    }

    println!("{ans}")
}
