pub fn day01() {
    let file = std::fs::read_to_string("private/day1.txt")
        .expect("There should be a file private/day1.txt");
    let mut elf_totals = file
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(|el| el.parse::<u32>().unwrap_or(0))
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();

    elf_totals.sort_by(|a, b| b.cmp(a));

    println!(
        "part1: {}\npart2: {}",
        elf_totals[0],
        elf_totals[..3].into_iter().sum::<u32>()
    );
}

///////////////////////////////////////////////////////////////////////////////

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn _day1_version1() {
    let file = File::open("private/day1.txt").expect("There should be a file src/day1/cals.txt");
    let reader = BufReader::new(file);
    let mut cal_sum_vec = Vec::new();

    let mut elf_cals = 0;
    for line in reader.lines() {
        let line = line.expect("there should be a line to read");
        let line = line.trim();
        if line == "" {
            cal_sum_vec.push(elf_cals);
            elf_cals = 0;
            continue;
        }
        elf_cals += line
            .parse::<i32>()
            .expect("each line should contain nothing or an i32");
    }

    cal_sum_vec.sort();
    cal_sum_vec.reverse();
    let sum_top3 = cal_sum_vec[0] + cal_sum_vec[1] + cal_sum_vec[2];

    println!("part 1: {}\npart 2: {}", cal_sum_vec[0], sum_top3);
}
