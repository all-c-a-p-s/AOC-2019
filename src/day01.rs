pub fn part_one() -> i32 {
    include_str!("day01.in")
        .lines()
        .map(|x| x.trim().parse::<i32>().unwrap())
        .map(|x| x / 3 - 2)
        .sum()
}

fn calc_fuel(x: i32) -> i32 {
    match x {
        p if p <= 8 => 0,
        k => {
            let n = k / 3 - 2;
            n + calc_fuel(n)
        }
    }
}

pub fn part_two() -> i32 {
    include_str!("day01.in")
        .lines()
        .map(|x| x.trim().parse::<i32>().unwrap())
        .map(calc_fuel)
        .sum()
}
