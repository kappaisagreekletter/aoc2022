use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn day01() {
    let file = File::open("src/day1/cals.txt").expect("There should be a file src/day1/cals.txt");
    let reader = BufReader::new(file);
    let mut cal_sum_vec = Vec::new();

    let mut elf_cals = 0;
    for line in reader.lines() {
        let line = line.expect("there should be a line to read");
        let line = line.trim();
        if line == "" {
            cal_sum_vec.push(elf_cals);
            elf_cals = 0;
            //println!();
            continue;
        }
        elf_cals += line
            .parse::<i32>()
            .expect("each line should contain nothing or an i32");

        //println!("{}", line);
    }
    //println!("{cal_sum_vec:#?}");

    cal_sum_vec.sort();
    cal_sum_vec.reverse();
    let sum_top3 = cal_sum_vec[0] + cal_sum_vec[1] + cal_sum_vec[2];

    println!("{}", cal_sum_vec[0]);
    println!("{}", sum_top3);
}
