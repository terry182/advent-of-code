use std::io;

fn valid(target: u64, numbers: Vec<u64>, concat: bool) -> bool {
    let mut stack: Vec<(u64, usize)> = Vec::new();
    let n = numbers.len();

    stack.push((numbers[0], 1));
    while !stack.is_empty() {
        let (cur, idx) = stack.pop().unwrap();
        if (idx == n && cur != target) || cur > target {
            continue;
        } else if idx == n {
            return true;
        } else {
            stack.push((cur + numbers[idx], idx + 1));
            stack.push((cur * numbers[idx], idx + 1));
            if concat {
                stack.push((format!("{}{}", cur, numbers[idx]).parse().unwrap(), idx + 1));
            }
        }
    }
    false
}

fn main() {
    let stdin = io::stdin();
    let mut sum: u64 = 0;
    let mut sum2: u64 = 0;
    for line in stdin.lines() {
        let (target, nums) = line.as_ref().unwrap().split_once(':').unwrap();
        let t = target.parse::<u64>().unwrap();
        if valid(t, nums.trim().split(' ').map(|x| x.parse().unwrap()).collect(), false) {
            sum += target.parse::<u64>().unwrap();
        }
        if valid(t, nums.trim().split(' ').map(|x| x.parse().unwrap()).collect(), true) {
            sum2 += target.parse::<u64>().unwrap();
        }
    }

    println!("{sum}");
    println!("{sum2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let mut sum: u64 = 0;
        for line in input.lines() {
            let (target, nums) = line.split_once(':').unwrap();
            if valid(
                target.parse().unwrap(),
                nums.trim().split(' ').map(|x| x.parse().unwrap()).collect(),
                false,
            ) {
                sum += target.parse::<u64>().unwrap();
            }
        }
        assert_eq!(sum, 3749);
    }

    #[test]
    fn test_part2() {
        let input = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let mut sum: u64 = 0;
        for line in input.lines() {
            let (target, nums) = line.split_once(':').unwrap();
            if valid(
                target.parse().unwrap(),
                nums.trim().split(' ').map(|x| x.parse().unwrap()).collect(),
                true,
            ) {
                sum += target.parse::<u64>().unwrap();
            }
        }
        assert_eq!(sum, 11387);
    }
}
