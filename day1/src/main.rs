use std::fs;

fn main() {
    let file_content = fs::read_to_string("input.txt").expect("read an input.txt");

    let mut output: Vec<u32> = file_content
        .lines()
        .fold(vec![vec![]], |mut acc, line| {
            if line.is_empty() {
                acc.push(Vec::new());
            } else {
                acc.last_mut().unwrap().push(line.parse::<u32>().unwrap());
            }
            acc
        })
        .iter()
        .map(|elf_calories| elf_calories.iter().sum::<u32>())
        .collect();

    output.sort_by(|a, b| b.cmp(a));

    println!("Top 1 elf has {} calories", part_1(&output));
    println!("Top 3 elfs have {} calories", part_2(&output));
}

fn part_1(output: &[u32]) -> u32 {
    *output.first().unwrap()
}

fn part_2(output: &[u32]) -> u32 {
    output.iter().take(3).sum()
}
