pub fn day01() {
    println!("\nday 1:");

    let file = std::fs::read_to_string("inputs/day1.txt")
        .expect("There should be a file inputs/day1.txt");

    let mut elf_totals = file
        .split("\n\n")
        .map(|elf| {
            elf
                .split("\n")
                .map(|el| el.parse::<u32>().unwrap_or(0))
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();

    elf_totals.sort_by(|a, b| b.cmp(a));

    println!(
        "\tpart1: {}\n\tpart2: {}",
        elf_totals[0],
        elf_totals[..3].into_iter().sum::<u32>()
    );
}
