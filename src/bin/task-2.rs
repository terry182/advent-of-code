use std::io;

fn rules(a: i32, b: i32, dir:bool) -> bool {
    return a == b || ((b > a) != dir) || b.abs_diff(a) > 3;
}

fn main() {
    let stdin = io::stdin();
    let mut cnt = 0;
    for line in stdin.lines() {
        let s = line.unwrap();
        let mut nums = s
            .split_whitespace()
            .map(str::parse::<i32>)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let mut dir;
        let mut vio = 0;
        let mut flag = 3;
        // remove first one
        dir = nums[2] > nums[1];
        for i in 2..nums.len() {
            if rules(nums[i-1], nums[i], dir) {
                flag -= 1;
                break;
            }
        }

        // remove previous one
        dir = nums[1] > nums[0];
        for i in 1..nums.len() {
            if rules(nums[i-1], nums[i], dir) 
            {
                if vio == 1 {
                    flag -= 1;
                    break;
                }
                vio = 1;
                if i == 1 {
                    dir = nums[2] > nums[1];
                } else if i == 2 {
                    dir = nums[2] > nums[0];
                    if rules(nums[0], nums[2], dir) {
                        flag -= 1;
                        break;
                    }
                } else if i > 2 && rules(nums[i-2], nums[i], dir) {
                    flag -= 1;
                    break;
                }
            }
        }
        // clear
        vio = 0;
        dir = nums[1] > nums[0];

        // remove itself
        for i in 1..nums.len() {
            if rules(nums[i-1], nums[i], dir) 
            {
                if vio == 1 {
                    flag -= 1;
                    break;
                }
                vio = 1;
                if i == 1 {
                    dir = nums[2] > nums[0]
                }
                nums[i] = nums[i-1];
            }
        }
        cnt += (flag > 0) as i32;
    }
    println!("{cnt}");
}
