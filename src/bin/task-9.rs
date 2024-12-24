use std::collections::{BTreeMap, HashSet};
use std::io;

fn checksum(s: Vec<(i64, i64)>) -> i64 {
    let p: Vec<i64> = s
        .iter()
        .scan(0, |cur, &(id, len)| {
            let mem = cur.clone();
            *cur += len;
            if id != -1 {
                Some(id * (mem * len + (len - 1) * len / 2))
            } else {
                Some(0)
            }
        })
        .collect();
    p.iter().sum()
}

fn defrag(s: &String) -> Vec<(i64, i64)> {
    let mut s: Vec<(i64, i64)> = s
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let num = c.to_digit(10).unwrap();
            if i % 2 == 0 {
                (i as i64 / 2, num as i64)
            } else {
                (-1, num as i64)
            }
        })
        .collect();

    let mut last = s.len() - 1;
    let mut front = 0;
    let mut after = Vec::new();
    while front <= last {
        let (id, len) = s[front];
        if id == -1 {
            let (lid, last_len) = s[last];
            if lid == -1 {
                last -= 1;
                s.pop();
            } else {
                match last_len {
                    l if l < len => {
                        s[front] = (id, len - l);
                        last -= 1;
                        s.pop();
                        after.push((lid, l))
                    }
                    l if l == len => {
                        front += 1;
                        last -= 1;
                        s.pop();
                        after.push((lid, len))
                    }
                    l if l > len => {
                        front += 1;
                        s[last] = (lid, l - len);
                        after.push((lid, len))
                    }
                    _ => panic!(""),
                };
            }
        } else {
            after.push((id, len));
            front += 1;
        }
    }
    after
}

fn defrag_whole(s: &String) -> Vec<(i64, i64)> {
    let s: Vec<(i64, i64)> = s
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let num = c.to_digit(10).unwrap();
            if i % 2 == 0 {
                (i as i64 / 2, num as i64)
            } else {
                (-1, num as i64)
            }
        })
        .filter(|(_, c)| *c != 0)
        .collect();
    let mut spaces: BTreeMap<usize, i64> = BTreeMap::from_iter(
        s.iter().filter(|(x, _)| *x == -1).copied().map(|(_, y)| y).enumerate(),
    );
    let mut filling = Vec::new();
    for i in 0..s.len() {
        let (id, len) = s[s.len() - i - 1];
        if id == -1 {
            continue;
        }
        for (space_id, space_len) in spaces.clone() {
            if space_len > len {
                filling.push((space_id, (id, len)));
                spaces.insert(space_id, space_len - len);
                break;
            } else if space_len == len {
                filling.push((space_id, (id, len)));
                spaces.remove(&space_id);
                break;
            }
        }
    }
    println!("moved: {}", filling.len());

    filling.sort_by_key(|(sid, (id, _))| (*sid, -*id));

    let mut filled = HashSet::new();
    let mut finale = Vec::new();
    let mut space_id = 0;
    let mut filling_id = 0;
    for (id, len) in s {
        if id != -1 && !filled.contains(&id) {
            finale.push((id, len));
            filled.insert(id);
        } else if id != -1 {
            finale.push((-1, len));
        } else {
            let mut leftover = len;
            while filling_id < filling.len() {
                let (si, (tid, tlen)) = filling[filling_id];
                if space_id == si {
                    if !filled.contains(&tid) {
                        finale.push((tid, tlen));
                        filled.insert(tid);
                        leftover -= tlen;
                    }
                    filling_id += 1;
                } else {
                    break;
                }
            }
            if leftover > 0 {
                finale.push((-1, leftover));
            }
            space_id += 1;
        }
    }
    finale
}

fn main() {
    let uncompacted = io::stdin().lines().next().unwrap().unwrap();
    println!("{}", checksum(defrag(&uncompacted)));
    println!("{}", checksum(defrag_whole(&uncompacted)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum() {
        let t = vec![
            (0, 2),
            (9, 2),
            (8, 1),
            (1, 3),
            (8, 3),
            (2, 1),
            (7, 3),
            (3, 3),
            (6, 1),
            (4, 2),
            (6, 1),
            (5, 4),
            (6, 2),
        ];
        assert_eq!(checksum(t), 1928);
    }
    #[test]
    fn test_input() {
        let s = "2333133121414131402";
        let v = defrag(&s.to_string());
        println!(
            "{}",
            v.iter()
                .map(|(x, y)| format!("({x},{y})").to_string())
                .collect::<Vec<String>>()
                .join(",\n")
        );
        assert_eq!(checksum(v), 1928);
    }

    #[test]
    fn test_defrag_whole() {
        let s = "2333133121414131402";
        let v = defrag_whole(&s.to_string());
        println!(
            "{}",
            v.iter()
                .map(|(x, y)| format!("({x},{y})").to_string())
                .collect::<Vec<String>>()
                .join(",\n")
        );
        assert_eq!(checksum(v), 2858);
    }
}
