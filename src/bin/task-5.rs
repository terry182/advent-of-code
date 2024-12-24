use std::io;

fn solve(mut arr: Vec<Vec<char>>) -> i32 {
    let m = arr.len();
    let n = arr[0].len();
    let mut posx: isize = 0;
    let mut posy: isize = 0;
    'search: for i in 0..m {
        for j in 0..n {
            if arr[i][j] == '^' {
                posx = i as isize;
                posy = j as isize;
                break 'search;
            }
        }
    }

    let dx: &[isize] = &[0, 1, 0, -1];
    let dy: &[isize] = &[1, 0, -1, 0];
    let mut dir = 3; // up
    let mut cnt = 0;
    let oob = |x: isize, y: isize| x < 0 || x >= (m as isize) || y < 0 || y >= (n as isize);
    loop {
        if oob(posx, posy) {
            break;
        }
        if arr[posx as usize][posy as usize] != 'X' {
            arr[posx as usize][posy as usize] = 'X';
            cnt += 1;
        }
        // peek
        if !oob(posx + dx[dir], posy + dy[dir]) {
            let newx: usize = (posx + dx[dir]) as usize;
            let newy: usize = (posy + dy[dir]) as usize;
            if arr[newx][newy] == '#' {
                dir = (dir + 1) % 4;
            }
        } else {
            break;
        }

        posx += dx[dir];
        posy += dy[dir];
    }
    cnt
}

fn solve2(arr: Vec<Vec<char>>) -> u32 {
    1
}

fn main() {
    let stdin = io::stdin();
    let arr: Vec<Vec<char>> = stdin
        .lines()
        .map(|x| x.unwrap().chars().collect())
        .collect();
    let cnt = solve(arr.clone());
    let cnt2 = solve2(arr);
    println!("{cnt}");
    println!("{cnt2}");
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_toy() {
        let input = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let t: Vec<Vec<char>> = input
            .lines()
            .into_iter()
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|x| x.chars().collect())
            .collect();
        assert_eq!(solve(t.clone()), 41);
        assert_eq!(solve2(t.clone()), 6);
    }
}
