// can combine parts 1 and 2 but decided not to

pub fn day04() {
    println!("\nday 4:");

    let file =
        std::fs::read_to_string("inputs/day4.txt").expect("There should be a file inputs/day4.txt");

    // part 1
    let contained_count = file
        .split("\n")
        .map(|line| {
            let split_line = line.split(",").collect::<Vec<&str>>();
            if check_contained(split_line, false) {
                1
            } else {
                0
            }
        })
        .sum::<u32>();

    println!("\tpart1: {contained_count:#?}");

    // part 2
    let contained_count = file
        .split("\n")
        .map(|line| {
            let split_line = line.split(",").collect::<Vec<&str>>();
            if check_contained(split_line, true) {
                1
            } else {
                0
            }
        })
        .sum::<u32>();

    println!("\tpart2: {contained_count:#?}");
}

fn check_contained(line: Vec<&str>, fully_contained: bool) -> bool {
    if line[0] == "" {
        return false;
    }

    let first = line[0]
        .split("-")
        .map(|el| el.parse::<u16>().expect("Should be valid numbers"))
        .collect::<Vec<u16>>();

    let second = line[1]
        .split("-")
        .map(|el| el.parse::<u16>().expect("Should be valid numbers"))
        .collect::<Vec<u16>>();

    if (!fully_contained && first[0] <= second[0] && first[1] >= second[1])
        || (first[0] >= second[0] && first[1] <= second[1])
    {
        return true;
    }
    if fully_contained && first[0] <= second[1] && first[1] >= second[0] {
        return true;
    }
    false
}
