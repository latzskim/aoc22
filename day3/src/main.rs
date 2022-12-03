use std::{collections::HashSet, fs};

fn main() {
    let backpack_items = fs::read_to_string("input.txt").unwrap();

    let part_1_sum: u32 = part_1_sum(backpack_items.clone());
    println!("part_1_sum: {}", part_1_sum);

    let part_2_sum: u32 = part_2_sum(backpack_items);
    println!("part_2_sum: {}", part_2_sum);
}

fn part_1_sum(backpack_items: String) -> u32 {
    backpack_items
        .lines()
        .map(split_into_compartments)
        .map(|(left, right)| find_duplicates(left, right))
        .map(sum)
        .sum()
}

fn part_2_sum(backpack_items: String) -> u32 {
    let lines: Vec<&str> = backpack_items.lines().collect();
    lines
        .chunks(3)
        .map(|chunk| find_duplicate_in_group(chunk.into()))
        .map(sum)
        .sum()
}

// Assume that it's always divisible by 2.
fn split_into_compartments<'a>(input_line: &'a str) -> (&'a str, &'a str) {
    (
        &input_line[..input_line.len() / 2],
        &input_line[input_line.len() / 2..],
    )
}

fn find_duplicates(left: &str, right: &str) -> HashSet<char> {
    left.chars()
        .filter(|c| right.chars().any(|c1| *c == c1))
        .collect()
}

fn find_duplicate_in_group(group: Box<[&str]>) -> HashSet<char> {
    group[0]
        .chars()
        .filter(|c| group[1].chars().any(|c1| *c == c1))
        .filter(|c| group[2].chars().any(|c2| *c == c2))
        .collect()
}

fn sum(duplicates: HashSet<char>) -> u32 {
    duplicates
        .iter()
        .filter(|c| c.is_uppercase())
        .map(|c| (*c as u32) - 38_u32)
        .sum::<u32>()
        + duplicates
            .iter()
            .filter(|c| c.is_lowercase())
            .map(|c| (*c as u32) - 96_u32)
            .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_input_line_in_half() {
        const DATA: &str = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let (left, right) = split_into_compartments(DATA);
        assert_eq!("vJrwpWtwJgWr", left);
        assert_eq!("hcsFMMfFFhFp", right);
    }

    #[test]
    fn empty_vector_when_no_duplicates() {
        let (left, right) = ("a", "b");
        let duplicates = find_duplicates(left, right);
        assert!(duplicates.is_empty());
    }

    #[test]
    fn list_of_duplicates() {
        let (left, right) = ("abc", "cbe");
        let duplicates = find_duplicates(left, right);
        assert_eq!(HashSet::from(['b', 'c']), duplicates);
    }

    #[test]
    fn list_of_duplicates_contains_unique_values() {
        let (left, right) = ("abcbbbb", "cbebbcc");
        let duplicates = find_duplicates(left, right);
        assert_eq!(HashSet::from(['b', 'c']), duplicates);
    }

    #[test]
    fn sum_chars() {
        assert_eq!(1, sum(HashSet::from(['a'])));
        assert_eq!(26, sum(HashSet::from(['z'])));
        assert_eq!(27, sum(HashSet::from(['A'])));
        assert_eq!(52, sum(HashSet::from(['Z'])));
        assert_eq!(53, sum(HashSet::from(['Z', 'a'])));
    }

    #[test]
    fn sum_of_duplicates_part_1() {
        const DATA: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(157, part_1_sum(DATA.to_string()));
    }

    #[test]
    fn find_duplicate_in_a_group() {
        let group1: Box<[&str]> = Box::new([
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
        ]);
        let group2: Box<[&str]> = Box::new([
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ]);

        assert_eq!(HashSet::from(['r']), find_duplicate_in_group(group1));
        assert_eq!(HashSet::from(['Z']), find_duplicate_in_group(group2));
    }

    #[test]
    fn sum_of_duplicates_in_group_part_2() {
        const DATA: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(70, part_2_sum(DATA.to_string()));
    }
}
