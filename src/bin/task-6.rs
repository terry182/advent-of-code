use std::io;
#[derive(PartialEq, Eq)]
enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

fn next(d: &Direction) -> Direction {
    match d {
        Direction::Top => Direction::Right,
        Direction::Right => Direction::Bottom,
        Direction::Bottom => Direction::Left,
        Direction::Left => Direction::Top,
    }
}

fn walk(arr: &Vec<Vec<char>>, cx: usize, cy: usize, dir: &mut Direction) -> Option<(usize, usize)> {
    let m = arr.len();
    let n = arr[0].len();
    let oob = |x, y| x >= m || y >= n;
    while let Some((nx, ny)) = match dir {
        Direction::Top => cx.checked_sub(1).map(|x| (x, cy)),
        Direction::Bottom => Some((cx + 1, cy)),
        Direction::Left => cy.checked_sub(1).map(|y| (cx, y)),
        Direction::Right => Some((cx, cy + 1)),
    } {
        if oob(nx, ny) {
            return None;
        }
        if arr[nx][ny] == '#' {
            *dir = next(dir);
            continue;
        }
        return Some((nx, ny));
    }
    None
}

fn cal_path(arr: Vec<Vec<char>>) -> i32 {
    let mut p = arr.clone();
    let m = arr.len();
    let n = arr[0].len();
    let (mut cx, mut cy) = (0, 0);
    for i in 0..m {
        for j in 0..n {
            if arr[i][j] == '^' {
                cx = i;
                cy = j;
            }
        }
    }

    let mut cnt = 0;
    let mut dir = Direction::Top;
    loop {
        match walk(&p, cx, cy, &mut dir) {
            Some((nx, ny)) => {
                cx = nx;
                cy = ny;
            }
            _ => break,
        }
        if p[cx][cy] != 'X' {
            cnt += 1;
        }
        p[cx][cy] = 'X';
    }
    cnt
}

fn check_loop(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let (mut sx, mut sy) = (x, y); // fast
    let mut sdir = Direction::Top;
    let (mut fx, mut fy) = (x, y); // fast
    let mut fdir = Direction::Top;

    loop {
        if let Some((nx, ny)) = walk(grid, sx, sy, &mut sdir) {
            sx = nx;
            sy = ny;
        } else {
            return false;
        }
        if let Some((nx, ny)) = walk(grid, fx, fy, &mut fdir) {
            fx = nx;
            fy = ny;
        } else {
            return false;
        }
        if let Some((nx, ny)) = walk(grid, fx, fy, &mut fdir) {
            fx = nx;
            fy = ny;
        } else {
            return false;
        }
        if sx == fx && sy == fy && sdir == fdir {
            break;
        }
    }

    return true;
}
fn cal_block(grid: Vec<Vec<char>>) -> u32 {
    let mut p = grid.clone();
    let m = grid.len();
    let n = grid[0].len();

    let (mut cx, mut cy) = (0, 0);
    for i in 0..m {
        for j in 0..n {
            if p[i][j] == '^' {
                cx = i;
                cy = j;
            }
        }
    }

    let mut ans = 0;

    for i in 0..m {
        for j in 0..n {
            if p[i][j] == '.' {
                p[i][j] = '#';
                if check_loop(&p, cx, cy) {
                    ans += 1;
                }
                p[i][j] = '.';
            }
        }
    }
    ans
}

fn main() {
    let stdin = io::stdin();
    let grid: Vec<Vec<char>> = stdin.lines().map(|x| x.unwrap().chars().collect()).collect();

    let ans = cal_path(grid.clone());
    let ans2 = cal_block(grid);
    println!("{ans}");
    println!("{ans2}");
}
