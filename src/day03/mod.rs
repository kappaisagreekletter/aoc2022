pub fn day03() {
    println!("\nday 3:");

    let file =
        std::fs::read_to_string("inputs/day3.txt").expect("There should be a file inputs/day3.txt");

    println!("\tpart1: {}", get_sum_priority_of_lines(&file));
    println!("\tpart2: {}", get_badges_prio_sum(&file));
}

fn get_priority(c: char) -> u16 {
    let c = c as u16;
    match c {
        65..=90 => c - 64 + 26,
        97..=122 => c - 96,
        _ => panic!("Can't convert this type"),
    }
}

fn get_sum_priority_of_lines(lines: &str) -> u32 {
    lines
        .split("\n")
        .map(|rucksack| {
            if rucksack == "" {
                return 0;
            }
            let (left, right) = rucksack.split_at(rucksack.len() / 2);
            for c in left.chars() {
                if right.contains(c) {
                    return get_priority(c) as u32;
                }
            }
            panic!("no duplicates found in rucksack: {rucksack}");
        })
        .sum()
}

fn get_badges_prio_sum(lines: &str) -> u32 {
    let split_file: Vec<&str> = lines.split("\n").collect();
    let mut sum_badges: u32 = 0;
    let mut three_rucksacks: Vec<&str> = Vec::new();
    let mut i = 0;
    for rucksack in split_file {
        i += 1;
        if i % 3 == 0 {
            for c in rucksack.chars() {
                if three_rucksacks[0].contains(c) && three_rucksacks[1].contains(c) {
                    sum_badges += get_priority(c) as u32;
                    three_rucksacks = Vec::new();
                    break;
                }
            }
        } else {
            three_rucksacks.push(rucksack);
        }
    }
    sum_badges
}
