mod day01;
mod day02;

pub fn run(day: u8) {
    match day {
        1 => {
            println!("Day 1");
            let data = day01::Data::init();
            println!("Part A: {}", day01::part_a(&data));
            println!("Part B: {}", day01::part_b(&data));
        }
        2 => {
            println!("Day 2");
            let data = day02::Data::init();
            println!("Part A: {}", day02::part_a(&data));
            println!("Part B: {}", day02::part_b(&data));
        }
        _ => println!("Day {} not implemented yet", day),
    }
}