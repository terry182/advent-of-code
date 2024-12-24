use std::cmp::max;
use std::collections::HashSet;
use std::io;
fn solve(grid: &Vec<Vec<char>>) -> (usize, usize) {
    let m = grid.len();
    let n = grid[0].len();
    let oob = |(x, y)| x < 0 || x >= m as isize || y >= n as isize || y < 0;
    let sym = HashSet::<char>::from_iter(
        grid.iter().flat_map(|x| x.iter().copied()).filter(|c| *c != '.'),
    );
    let p: Vec<(usize, usize)> = (0..m).flat_map(|x| (0..n).map(move |y| (x, y))).collect();
    let mut points = HashSet::<(usize, usize)>::new();
    let mut updated = HashSet::<(usize, usize)>::new();
    for c in sym {
        let pos: Vec<(usize, usize)> =
            p.iter().filter(|(i, j)| grid[*i][*j] == c).copied().collect();
        for p1 in &pos {
            for p2 in &pos {
                if p1 != p2 {
                    let (li, lj) = (p1.0 as isize, p1.1 as isize);
                    let (ri, rj) = (p2.0 as isize, p2.1 as isize);

                    if !oob((ri + ri - li, rj + rj - lj)) {
                        points.insert(((ri + ri - li) as usize, (rj + rj - lj) as usize));
                    }

                    if !oob((li + li - ri, lj + lj - rj)) {
                        points.insert(((li + li - ri) as usize, (lj + lj - rj) as usize));
                    }

                    // part 2
                    let (di, dj) = (ri - li, rj - lj);
                    let t = max(m, n) as isize;
                    let l = (0..t).find(|x| oob((li - x*di, lj - x*dj))).unwrap();
                    let r = (0..t).find(|x| oob((li + x*di, lj + x*dj))).unwrap();
                    for i in -l..r {
                        if !oob((li+i*di, lj+i*dj)) {
                            updated.insert(((li+i*di) as usize, (lj+i*dj) as usize));
                        }
                    }
                }
            }
        }
    }
    (points.len(), updated.len())
}
fn main() {
    let grid: Vec<Vec<char>> = io::stdin().lines().map(|x| x.unwrap().chars().collect()).collect();
    let (ans, ans2) = solve(&grid);
    println!("{} {}", ans, ans2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_in() {
        let s = r"............
                ........0...
                .....0......
                .......0....
                ....0.......
                ......A.....
                ............
                ............
                ........A...
                .........A..
                ............
                ............";
        let grid: Vec<Vec<char>> = s.lines().map(|x| x.chars().collect()).collect();
        assert_eq!(solve(&grid), (14, 34));
    }
}
