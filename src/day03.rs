use std::collections::HashSet;

pub fn part_one() -> i32 {
    let lines = include_str!("day03.in")
        .lines()
        .map(String::from)
        .collect::<Vec<_>>();

    let &[ref one, ref two] = lines.as_slice() else {
        unreachable!("expected exactly 2 lines");
    };

    let vs_one = to_vecs(&one);
    let vs_two = to_vecs(&two);

    let h_one = visits(&vs_one);
    let h_two = visits(&vs_two);

    let intersects = h_one.intersection(&h_two);

    intersects.fold(vs_one.len().max(vs_two.len()) as i32, |acc, (x, y)| {
        if acc > x.abs() + y.abs() {
            x.abs() + y.abs()
        } else {
            acc
        }
    })
}

fn visits(vs: &[(i32, i32)]) -> HashSet<(i32, i32)> {
    let mut h = HashSet::new();
    let (mut x, mut y) = (0, 0);
    for (dx, dy) in vs {
        x += dx;
        y += dy;
        h.insert((x, y));
    }

    h
}

fn to_vecs(x: &String) -> Vec<(i32, i32)> {
    fn parse_instruction(instr: String) -> Vec<(i32, i32)> {
        let parse_num = move |x: &[char]| x.iter().collect::<String>().trim().parse().unwrap();
        let n_times = move |v: (i32, i32), n: usize| (0..n).map(|_| v).collect();
        match instr.chars().collect::<Vec<_>>().as_slice() {
            ['R', tail @ ..] => n_times((1, 0), parse_num(tail)),
            ['L', tail @ ..] => n_times((-1, 0), parse_num(tail)),
            ['U', tail @ ..] => n_times((0, 1), parse_num(tail)),
            ['D', tail @ ..] => n_times((0, -1), parse_num(tail)),
            _ => unreachable!(),
        }
    }

    x.split(",")
        .map(String::from)
        .map(parse_instruction)
        .fold(vec![], |acc, vs| vec![acc, vs].concat())
}

fn combined_distance(coord: (i32, i32), v1: &[(i32, i32)], v2: &[(i32, i32)]) -> i32 {
    let dist = move |vs: &[(i32, i32)]| vs.iter().take_while(|&x| *x != coord).count() as i32 + 1;
    dist(v1) + dist(v2)
}

fn sequenced_visits(vs: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let mut v = vec![];
    let (mut x, mut y) = (0, 0);
    for (dx, dy) in vs {
        x += dx;
        y += dy;
        v.push((x, y));
    }

    v
}

pub fn part_two() -> i32 {
    let lines = include_str!("day03.in")
        .lines()
        .map(String::from)
        .collect::<Vec<_>>();

    let &[ref one, ref two] = lines.as_slice() else {
        unreachable!("expected exactly 2 lines");
    };

    let vs_one = to_vecs(one);
    let vs_two = to_vecs(two);

    let h_one = visits(&vs_one);
    let h_two = visits(&vs_two);

    let intersects = h_one.intersection(&h_two);

    let seq_one = sequenced_visits(&vs_one);
    let seq_two = sequenced_visits(&vs_two);

    intersects.fold(vs_one.len().max(vs_two.len()) as i32 * 2, |acc, &x| {
        let d = combined_distance(x, &seq_one, &seq_two);
        if acc > d { d } else { acc }
    })
}
