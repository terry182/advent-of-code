use std::collections::{HashSet, VecDeque};
use std::io;

#[derive(PartialEq, Eq)]
enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

fn walk(bound: (usize, usize), pos: (usize, usize), dir: Direction) -> Option<(usize, usize)> {
    let (m, n) = bound;
    let (x, y) = pos;
    if let Some((nx, ny)) = match dir {
        Direction::Top => x.checked_sub(1).map(|t| (t, y)),
        Direction::Bottom => Some((x + 1, y)),
        Direction::Left => y.checked_sub(1).map(|t| (x, t)),
        Direction::Right => Some((x, y + 1)),
    } {
        if nx < m && ny < n {
            return Some((nx, ny));
        }
    }
    None
}

fn flow(
    arr: &Vec<Vec<i64>>,
    dp: &mut Vec<Vec<i64>>,
    rating: &mut Vec<Vec<i64>>,
    pos: (usize, usize),
) {
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    let bound = (arr.len(), arr[0].len());
    let mut set = HashSet::new();
    q.push_back(pos);
    while !q.is_empty() {
        let (curx, cury) = q.pop_front().unwrap();
        rating[curx][cury] += 1;
        set.insert((curx, cury));
        for dir in [Direction::Top, Direction::Bottom, Direction::Left, Direction::Right] {
            if let Some((nx, ny)) = walk(bound, (curx, cury), dir) {
                if arr[nx][ny] == arr[curx][cury] - 1 {
                    q.push_back((nx, ny));
                }
            }
        }
    }
    set.iter().map(|&(x, y)| dp[x][y] += 1).count();
}

fn solve(arr: &Vec<Vec<i64>>) -> (i64, i64) {
    let (m, n) = (arr.len(), arr[0].len());
    let mut dp = vec![vec![0; n]; m];
    let mut rating = vec![vec![0; n]; m];
    (0..m)
        .flat_map(|x| (0..n).map(move |y| (x, y)))
        .filter(|&(x, y)| arr[x][y] == 9)
        .map(|pos| flow(arr, &mut dp, &mut rating, pos))
        .count(); // use count to consume the iterator to force calculation

    (0..m)
        .flat_map(|x| (0..n).map(move |y| (x, y)))
        .filter(|&(x, y)| arr[x][y] == 0)
        .map(|(x, y)| (dp[x][y], rating[x][y]))
        .fold((0, 0), |(curx, cury), (x, y)| (curx + x, cury + y))
}
fn main() {
    let grid = io::stdin()
        .lines()
        .map(|x| x.unwrap().chars().map(|c| c.to_digit(10).unwrap() as i64).collect())
        .collect();
    println!("{}", solve(&grid).0);
    println!("{}", solve(&grid).1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_example() {
        let input = r"0123
1234
8765
9876";
        let grid: Vec<Vec<i64>> = input
            .lines()
            .map(|x| x.chars().map(|x| x.to_digit(10).unwrap() as i64).collect())
            .collect();
        assert_eq!(solve(&grid).0, 1);
    }

    #[test]
    fn large_example() {
        let input = r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        let grid: Vec<Vec<i64>> = input
            .lines()
            .map(|x| x.chars().map(|x| x.to_digit(10).unwrap() as i64).collect())
            .collect();
        assert_eq!(solve(&grid), (36, 81));
    }
}
