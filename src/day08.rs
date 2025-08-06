const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const LAYER_SIZE: usize = WIDTH * HEIGHT;

pub fn load_input() -> Vec<i32> {
    include_str!("day08.in")
        .trim()
        .chars()
        .map(|x| String::from(x).parse::<i32>().unwrap())
        .collect()
}

pub fn part_one() -> i32 {
    let data = load_input();

    let count_occurances = |v: &[i32], d: i32| v.iter().filter(|&x| *x == d).count();

    let mut min_zeros = LAYER_SIZE + 1;
    let mut score = 0;
    for i in 0..data.len() / LAYER_SIZE {
        let window = &data[i * LAYER_SIZE..i * LAYER_SIZE + LAYER_SIZE];

        let zeros = count_occurances(window, 0);
        if zeros < min_zeros {
            min_zeros = zeros;
            score = count_occurances(window, 1) * count_occurances(window, 2);
        }
    }

    score as i32
}

pub fn part_two() -> String {
    let data = load_input();

    let mut pixels = vec![vec![]; LAYER_SIZE];

    for i in 0..data.len() / LAYER_SIZE {
        let window = &data[i * LAYER_SIZE..i * LAYER_SIZE + LAYER_SIZE];
        for (i, &x) in window.iter().enumerate() {
            pixels[i].push(x);
        }
    }

    //pixels.iter_mut().for_each(|v| v.reverse());

    let mut message = "\n".to_string();

    let get_value = |v: Vec<i32>| v.into_iter().find(|x| *x != 2).unwrap();

    for row in 0..HEIGHT {
        for column in 0..WIDTH {
            let idx = row * WIDTH + column;
            let c = match get_value(pixels[idx].clone()) {
                0 => ".",
                1 => "#",
                _ => unreachable!("oops"),
            };
            message += c;
        }
        message += "\n";
    }

    message
}
