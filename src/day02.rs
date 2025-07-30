fn run_tape(a: i32, b: i32, v: &Vec<i32>) -> i32 {
    let mut tape = v.clone();
    tape[1] = a;
    tape[2] = b;

    for i in 0..tape.len() / 4 {
        let window = &tape.clone()[i * 4..i * 4 + 4];
        let f = match window[0] {
            1 => move |a: i32, b: i32| Some(a + b),
            2 => move |a: i32, b: i32| Some(a * b),
            99 => return tape[0],
            _ => unreachable!(),
        };

        let Some(res) = f(tape[window[1] as usize], tape[window[2] as usize]) else {
            return tape[0];
        };

        tape[window[3] as usize] = res;
    }
    tape[0]
}

pub fn part_one() -> i32 {
    let tape: Vec<i32> = include_str!("day02.in")
        .split(",")
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();
    run_tape(12, 2, &tape)
}

pub fn part_two() -> i32 {
    let original: Vec<i32> = include_str!("day02.in")
        .split(",")
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();
    for a in 0..100 {
        for b in 0..100 {
            if run_tape(a, b, &original) == 19690720 {
                return 100 * a + b;
            }
        }
    }
    unreachable!()
}
