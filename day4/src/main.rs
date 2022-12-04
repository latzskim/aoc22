use std::{fs, ops::Range};

fn main() {
    let elves_ranges = fs::read_to_string("input.txt").unwrap();

    let part_1_ranges = count_overlapses(&elves_ranges, Box::new(OverlapPart1));
    println!("part1: {}", part_1_ranges);

    let part_2_ranges = count_overlapses(&elves_ranges, Box::new(OverlapPart2));
    println!("part2: {}", part_2_ranges);
}

fn count_overlapses(elves_ranges: &str, ranger: Box<dyn Ranger>) -> usize {
    elves_ranges
        .lines()
        .map(|line| line.split(','))
        .map(|mut range| {
            (
                range_from(range.next().unwrap()),
                range_from(range.next().unwrap()),
            )
        })
        .filter(|(range1, range2)| ranger.overlap(range1, range2) || ranger.overlap(range2, range1))
        .count()
}

fn range_from(elf_range: &str) -> Range<i32> {
    let mut splitted = elf_range.split('-');

    Range {
        start: splitted.next().unwrap().parse::<i32>().unwrap(),
        end: splitted.next().unwrap().parse::<i32>().unwrap() + 1,
    }
}

trait Ranger {
    fn overlap(&self, range1: &Range<i32>, range2: &Range<i32>) -> bool;
}

struct OverlapPart1;

impl Ranger for OverlapPart1 {
    fn overlap(&self, range1: &Range<i32>, range2: &Range<i32>) -> bool {
        range1.contains(
            &range2
                .clone()
                .next()
                .expect("expected first element in range2"),
        ) && range1.contains(
            &range2
                .clone()
                .last()
                .expect("expected last element in range2"),
        )
    }
}

struct OverlapPart2;

impl Ranger for OverlapPart2 {
    fn overlap(&self, range1: &Range<i32>, range2: &Range<i32>) -> bool {
        range1.contains(
            &range2
                .clone()
                .next()
                .expect("expected first element in range2"),
        ) || range1.contains(
            &range2
                .clone()
                .last()
                .expect("expected last element in range2"),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn true_when_overlap_fully() {
        let range1 = 1..10;
        let range2 = 5..7;

        assert!(OverlapPart1.overlap(&range1, &range2));
        assert!(!OverlapPart1.overlap(&range2, &range1));
    }

    #[test]
    fn one_element_range_overlap_fully() {
        let range1 = 1..2;
        let range2 = 1..10;

        assert!(!OverlapPart1.overlap(&range1, &range2));
        assert!(OverlapPart1.overlap(&range2, &range1));
    }

    #[test]
    fn should_create_ranges() {
        const DATA1: &str = "1-1";
        const DATA2: &str = "1-10";
        assert_eq!(1..2, range_from(DATA1));
        assert_eq!(1..11, range_from(DATA2));
    }

    #[test]
    fn part_1_count_overlapses() {
        const DATA: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

        assert_eq!(2, count_overlapses(DATA, Box::new(OverlapPart1)));
    }

    #[test]
    fn should_overlap_partialy() {
        let range1 = 1..10;
        let range2 = 8..15;

        assert!(OverlapPart2.overlap(&range1, &range2));
        assert!(OverlapPart2.overlap(&range2, &range1));
    }

    #[test]
    fn part_2_count_overlapses() {
        const DATA: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

        assert_eq!(4, count_overlapses(DATA, Box::new(OverlapPart2)));
    }
}
