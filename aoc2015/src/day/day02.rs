use super::{Day, Input, Types};

fn process(input: &Input) -> impl Iterator<Item = (i32, i32, i32)> + '_ {
    input.contents.lines().map(|line| {
        let dims = line
            .split('x')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        (dims[0], dims[1], dims[2])
    })
}

pub fn part_a(input: &Input) -> Types {
    let mut total = 0;
    for (l, w, h) in process(input) {
        let lw = l * w;
        let wh = w * h;
        let hl = h * l;
        let min = lw.min(wh).min(hl);
        total += 2 * lw + 2 * wh + 2 * hl + min;
    }
    Types::Number(total.try_into().unwrap())
}
pub fn part_b(input: &Input) -> Types {
    let mut total = 0;
    for (l, w, h) in process(input) {
        let way1 = 2 * l + 2 * w;
        let way2 = 2 * w + 2 * h;
        let way3 = 2 * h + 2 * l;
        let min = way1.min(way2).min(way3);
        total += l * w * h + min;
    }
    Types::Number(total.try_into().unwrap())
}

pub static DAY: Day = Day { part_a, part_b };
