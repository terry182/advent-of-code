use std::collections::VecDeque;
use std::io;
#[derive(PartialEq, Eq)]
enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

fn step(cx: usize, cy: usize, dir: Direction) -> Option<(usize, usize)> {
    match dir {
        Direction::Top => cx.checked_sub(1).map(|x| (x, cy)),
        Direction::Bottom => Some((cx + 1, cy)),
        Direction::Left => cy.checked_sub(1).map(|y| (cx, y)),
        Direction::Right => Some((cx, cy + 1)),
    }
}

fn bfs(arr: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<Vec<i32>> {
    let m = arr.len();
    let n = arr[0].len();
    let mut d = vec![vec![-1; n]; m];
    let mut q = VecDeque::<(usize, usize, i32)>::new();
    q.push_back((x, y, 0));
    let oob = |x, y| x >= m || y >= n;
    while !q.is_empty() {
        let (cx, cy, dis) = q.pop_front().unwrap();
        d[cx][cy] = dis;
        for dir in [Direction::Top, Direction::Right, Direction::Bottom, Direction::Left] {
            if let Some((nx, ny)) = step(cx, cy, dir) {
                if !oob(nx, ny) && arr[nx][ny] != '#' && d[nx][ny] == -1 {
                    q.push_back((nx, ny, dis + 1));
                }
            }
        }
    }
    d
}
fn main() {
    let grid: Vec<Vec<char>> =
        io::stdin().lines().map(|x| x.expect("unexpcted").chars().collect()).collect();

    let m = grid.len();
    let n = grid[0].len();
    let (sx, sy) = (0..m)
        .map(|x| (0..n).map(move |y| (x, y)))
        .flatten()
        .find(|&(x, y)| grid[x][y] == 'S')
        .unwrap();
    let (ex, ey) = (0..m)
        .map(|x| (0..n).map(move |y| (x, y)))
        .flatten()
        .find(|&(x, y)| grid[x][y] == 'E')
        .unwrap();

    let dis_start = bfs(&grid, sx, sy);
    let dis_end = bfs(&grid, ex, ey);
    let oob = |x, y| x >= m || y >= n;

    let ans = (0..m)
        .map(|x| (0..n).map(move |y| (x, y)))
        .flatten()
        .filter(|&(x, y)| grid[x][y] == '#')
        .filter(|&(x, y)| {
            let pts: Vec<(usize, usize)> =
                [Direction::Top, Direction::Right, Direction::Bottom, Direction::Left]
                    .into_iter()
                    .filter_map(|dir| step(x, y, dir))
                    .collect();
            for &(ix, iy) in &pts {
                for &(px, py) in &pts {
                    if !oob(ix, iy)
                        && dis_start[ix][iy] != -1
                        && !oob(px, py)
                        && dis_end[px][py] != -1
                        && dis_start[ix][iy] + dis_end[px][py] + 102 <= dis_start[ex][ey]
                    {
                        return true;
                    }
                }
            }
            false
        })
        .count();

    // part2
    let pts: Vec<(usize, usize)> = (0..m).map(|x| (0..n).map(move |y| (x, y))).flatten().collect();

    let ans2 = pts
        .iter()
        .map(|&x| pts.iter().map(move |&y| (x, y)))
        .flatten()
        .filter(|&((li, lj), (ri, rj))| li.abs_diff(ri) + lj.abs_diff(rj) <= 20)
        .filter(|&((li, lj), (ri, rj))| {
            let d: i32 = i32::try_from(li.abs_diff(ri) + lj.abs_diff(rj)).unwrap();
            dis_start[li][lj] != -1
                && dis_end[ri][rj] != -1
                && dis_start[li][lj] + dis_end[ri][rj] + 100 + d <= dis_start[ex][ey]
        })
        .count();

    println!("{ans}");
    println!("{ans2}");
}
