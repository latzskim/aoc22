use std::{collections::HashSet, fs};

fn main() {
    let datastream = fs::read_to_string("input.txt").unwrap();

    println!("{}", find_index(datastream.clone(), 4));
    println!("{}", find_index(datastream, 14));
}

fn find_index(datastream: String, len_of_chars: usize) -> usize {
    datastream
        .chars()
        .enumerate()
        .find(|(index, _)| {
            let x: HashSet<char> = datastream
                .clone()
                .chars()
                .skip(*index)
                .take(len_of_chars)
                .collect();
            x.len() == len_of_chars
        })
        .unwrap()
        .0
        + len_of_chars
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_index_offset_4() {
        const DATA1: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        const DATA2: &str = "nppdvjthqldpwncqszvftbrmjlhg";
        const DATA3: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        const DATA4: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

        assert_eq!(5, find_index(DATA1.to_string(), 4));
        assert_eq!(6, find_index(DATA2.to_string(), 4));
        assert_eq!(10, find_index(DATA3.to_string(), 4));
        assert_eq!(11, find_index(DATA4.to_string(), 4));
    }

    #[test]
    fn should_find_index_offset_14() {
        const DATA1: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        const DATA2: &str = "nppdvjthqldpwncqszvftbrmjlhg";
        const DATA3: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        const DATA4: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

        assert_eq!(23, find_index(DATA1.to_string(), 14));
        assert_eq!(23, find_index(DATA2.to_string(), 14));
        assert_eq!(29, find_index(DATA3.to_string(), 14));
        assert_eq!(26, find_index(DATA4.to_string(), 14));
    }
}
