use std::collections::HashMap;
use std::io;
use std::iter::zip;
fn keypad_code(s: &String) -> String {
    let keypad = HashMap::from([
        ('0', (3, 1)),
        ('A', (3, 2)),
        ('1', (2, 0)),
        ('2', (2, 1)),
        ('3', (2, 2)),
        ('4', (1, 0)),
        ('5', (1, 1)),
        ('6', (1, 2)),
        ('7', (0, 0)),
        ('8', (0, 1)),
        ('9', (0, 2)),
    ]);
    // Starts in A
    let mut curx = 3;
    let mut cury = 2;
    s.chars().map(|x| keypad[&x]).fold(String::new(), |s, (tx, ty)| {
        let mut p = s.clone();

        if curx == 3 && tx != 3 && ty == 0 {
            p.push_str("^".repeat(curx - tx).as_str());
            p.push_str("<".repeat(cury).as_str());
            curx = tx;
            cury = ty;
        } else if curx != 3 && tx == 3 && cury == 0 {
            p.push_str(">".repeat(ty).as_str());
            p.push_str("v".repeat(tx - curx).as_str());
            curx = tx;
            cury = ty;
        }

        if cury > ty {
            p.push_str("<".repeat(cury - ty).as_str());
        }

        if tx > curx {
            p.push_str("v".repeat(tx - curx).as_str());
        } else {
            p.push_str("^".repeat(curx - tx).as_str());
        }

        if ty > cury {
            p.push_str(">".repeat(ty - cury).as_str());
        }

        cury = ty;
        curx = tx;
        p.push('A');
        p
    })
}

fn control_code(s: String) -> String {
    let keypad =
        HashMap::from([('^', (0, 1)), ('A', (0, 2)), ('<', (1, 0)), ('v', (1, 1)), ('>', (1, 2))]);
    let mut cur = 'A';
    let mut flag = false;
    let seq = s
        .chars()
        .fold("".to_string(), |s, c| {
            let mut p = s;
            if cur == c {
                p.push('A');
            } else {
                if flag {
                    p.push('A');
                } else {
                    flag = true;
                }
                let (curx, cury) = keypad[&cur];
                let (tx, ty) = keypad[&c];
                if cur == '<' {
                    p.push_str(">".repeat(ty - cury).as_str());
                    p.push_str("^".repeat(curx - tx).as_str());
                } else if c == '<' {
                    p.push_str("v".repeat(tx - curx).as_str());
                    p.push_str("<".repeat(cury - ty).as_str());
                } else {
                    if ty < cury {
                        p.push_str("<".repeat(cury - ty).as_str());
                    }
                    if tx > curx {
                        p.push_str("v".repeat(tx - curx).as_str());
                    } else {
                        p.push_str("^".repeat(curx - tx).as_str());
                    }
                    if ty > cury {
                        p.push_str(">".repeat(ty - cury).as_str());
                    }
                }
                cur = c;
            }
            p
        })
        .to_string()
        + "A";
    seq
}

fn move_in_keypad(s: char, t: char) -> String {
    let keypad =
        HashMap::from([('^', (0, 1)), ('A', (0, 2)), ('<', (1, 0)), ('v', (1, 1)), ('>', (1, 2))]);
    let mut p = String::new();
    let (curx, cury) = keypad[&s];
            let (tx, ty) = keypad[&t];
            if s == '<' {
                p.push_str(">".repeat(ty - cury).as_str());
                p.push_str("^".repeat(curx - tx).as_str());
            } else if t == '<' {
                p.push_str("v".repeat(tx - curx).as_str());
                p.push_str("<".repeat(cury - ty).as_str());
            } else {
                if ty < cury {
                    p.push_str("<".repeat(cury - ty).as_str());
                }
                if tx > curx {
                    p.push_str("v".repeat(tx - curx).as_str());
                } else {
                    p.push_str("^".repeat(curx - tx).as_str());
                }
                if ty > cury {
                    p.push_str(">".repeat(ty - cury).as_str());
                }
            }
    p
}

fn markov_cal(s: &String, n: usize) -> i64 {
    let mut state = HashMap::<(char, char), i64>::new();
    let p = "A".to_string() + s.as_str();
    for (cur, next) in zip(p.chars(), s.chars()) {
        *state.entry((cur, next)).or_insert(0) += 1;
    }

    for _ in 0..n {
        let mut next_state = HashMap::<(char, char), i64>::new();
        for ((s, t), cnt) in state {
            let p = move_in_keypad(s, t);
            for e in zip(("A".to_string()+p.as_str()).chars(), (p+"A").as_str().chars()) {
                *next_state.entry(e).or_default() += cnt;
            }
        }
        state = next_state;
    }
    state.values().sum()
}

fn main() {
    let codes: Vec<String> = io::stdin().lines().map(|l| l.unwrap()).collect();
    let solve = |x: &String| control_code(control_code(keypad_code(x)));
    let mut ans: i64 = 0;
    let mut ans2: i64 = 0;
    for code in codes {
        let mut p = code.clone();
        p.pop();
        ans += p.parse::<i64>().unwrap() * (solve(&code).len() as i64);
        ans2 += p.parse::<i64>().unwrap() * markov_cal(&keypad_code(&code), 25);
    }
    println!("{ans}");
    println!("{ans2}");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn layering() {
        let key = String::from("029A");
        let sol = "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A";
        assert!(keypad_code(&key).len() == "<A^A>^^AvvvA".len());
        println!("{}", control_code(keypad_code(&key)));
        assert!(control_code(keypad_code(&key)).len() == "v<<A>>^A<A>AvA<^AA>A<vAAA>^A".len());
        assert!(control_code(control_code(keypad_code(&key))).len() == sol.len());
    }
    #[test]
    fn codes() {
        let keys = vec![
            ("029A", "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A"),
            ("980A", "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A"),
            ("179A", "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A"),
            ("456A", "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A"),
            ("379A", "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A"),
        ];
        let solve = |x| control_code(control_code(keypad_code(&x)));
        for (code, ans) in keys {
            assert_eq!(solve(code.to_string()).len(), ans.len());
        }
    }

    #[test]
    fn complexity() {
        let keys = vec![
            ("029A", "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A"),
            ("980A", "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A"),
            ("179A", "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A"),
            ("456A", "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A"),
            ("379A", "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A"),
        ];
        let solve = |x| control_code(control_code(keypad_code(&x)));
        let mut x: i64 = 0;
        for (code, _) in keys {
            let mut p = code.to_string();
            p.pop();
            x += p.parse::<i64>().unwrap() * (solve(code.to_string()).len() as i64);
        }
        assert_eq!(x, 126384);
    }
}
