use std::collections::HashMap;
use std::io;
fn solve(ints: &Vec<i64>, n: i64) -> i64 {
    let mut state = HashMap::<i64, i64>::new();
    for &i in ints {
        *state.entry(i).or_default() += 1;
    }

    // println!(
    //     "{}",
    //     state.iter().map(|(x, y)| format!("({x},{y})")).collect::<Vec<String>>().join(",")
    // );

    for _ in 0..n {
        let mut next_state = HashMap::<i64, i64>::new();

        for (num, cnt) in state {
            let t = match num {
                0 => vec![1],
                _ if num.to_string().len() % 2 == 0 => {
                    let str = num.to_string();
                    let l: usize = str.len() / 2;
                    let (s, t) = str.split_at(l);
                    // println!("split:{},{}", s, t);
                    vec![s.parse().unwrap(), t.parse().unwrap()]
                }
                _ => vec![num * 2024],
            };
            for val in t {
                *next_state.entry(val).or_default() += cnt;
            }
        }
        state = next_state;
        // println!(
        //     "{}",
        //     state.iter().map(|(x, y)| format!("({x},{y})")).collect::<Vec<String>>().join(",")
        // );
    }

    state.values().sum()
}
fn main() {
    let ints: Vec<i64> = io::stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();

    println!("{}", solve(&ints, 25));
    println!("{}", solve(&ints, 75));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_in() {
        let t = vec![125, 17];
        assert_eq!(solve(&t, 1), 3);
        assert_eq!(solve(&t, 2), 4);
        assert_eq!(solve(&t, 3), 5);
        assert_eq!(solve(&t, 4), 9);
        assert_eq!(solve(&t, 5), 13);
        assert_eq!(solve(&t, 6), 22);
        assert_eq!(solve(&t, 25), 55312);
    }
}
