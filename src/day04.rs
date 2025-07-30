pub fn part_one() -> i32 {
    let &[a, b] = include_str!("day04.in")
        .split("-")
        .map(|x| x.trim().parse().unwrap())
        .collect::<Vec<_>>()
        .as_slice()
    else {
        unreachable!("expected exactly 2 numbers in range")
    };
    (a..b).fold(0, |acc, x| acc + i32::from(ok(x)))
}

fn digits(mut x: u32) -> Vec<u32> {
    if x == 0 {
        return vec![0];
    }

    let mut digits = vec![];
    while x > 0 {
        digits.push(x % 10);
        x /= 10
    }

    digits.reverse();
    digits
}

fn ok(x: u32) -> bool {
    let ds = digits(x);
    let repeat = ds
        .windows(2)
        .fold(false, |acc, s| if s[0] == s[1] { true } else { acc });
    let non_decreasing = ds
        .windows(2)
        .fold(true, |acc, s| if s[1] < s[0] { false } else { acc });

    repeat && non_decreasing
}

fn ok2(x: u32) -> bool {
    let ds = digits(x);
    let l = ds.len();
    let non_decreasing = ds
        .windows(2)
        .fold(true, |acc, s| if s[1] < s[0] { false } else { acc });
    let repeat = ds.windows(4).fold(false, |acc, s| {
        if s[1] == s[2] && s[0] != s[1] && s[3] != s[2] {
            true
        } else {
            acc
        }
    }) || ds[0] == ds[1] && ds[1] != ds[2]
        || ds[l - 1] == ds[l - 2] && ds[l - 2] != ds[l - 3];
    non_decreasing && repeat
}

pub fn part_two() -> i32 {
    let &[a, b] = include_str!("day04.in")
        .split("-")
        .map(|x| x.trim().parse().unwrap())
        .collect::<Vec<_>>()
        .as_slice()
    else {
        unreachable!("expected exactly 2 numbers in range")
    };
    (a..b).fold(0, |acc, x| acc + i32::from(ok2(x)))
}
