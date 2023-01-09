use super::{Day, Input};

pub fn part_a(input: &Input) -> i32 {
    let mut total = 0;
    for line in input.contents.lines() {
        let dims = line
            .split('x')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let (l, w, h) = (dims[0], dims[1], dims[2]);
        let lw = l * w;
        let wh = w * h;
        let hl = h * l;
        let min = lw.min(wh).min(hl);
        total += 2 * lw + 2 * wh + 2 * hl + min;
    }
    total
}
pub fn part_b(input: &Input) -> i32 {
    let mut total = 0;
    for line in input.contents.lines() {
        let mut dims = line
            .split('x')
            .map(|s| s.to_string().parse::<i32>().unwrap());
        let (l, w, h) = [(); 3].map(|()| dims.next().unwrap());
        let way1 = 2 * l + 2 * w;
        let way2 = 2 * w + 2 * h;
        let way3 = 2 * h + 2 * l;
        let min = way1.min(way2).min(way3);
        total += l * w * h + min;
    }
    total
}

pub static DAY: Day = Day { part_a, part_b };
