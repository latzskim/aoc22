use std::fs;

fn main() {
    let battle_log = fs::read_to_string("input.txt").unwrap();

    let result_part_1 = calculate_result(&battle_log, Box::new(Part1Parser));
    println!("{}", result_part_1);

    let result_part_2 = calculate_result(&battle_log, Box::new(Part2Parser));
    println!("{}", result_part_2);
}

fn calculate_result(battle_log: &str, parser: Box<dyn LogParser>) -> u32 {
    battle_log
        .lines()
        .map(|log| parser.parse_log(log))
        .map(|(enemy, you)| {
            score_collector(vec![Box::new(battle(enemy, you.clone())), Box::new(you)])
        })
        .sum()
}

trait LogParser {
    fn parse_log(&self, log: &str) -> (HandGesture, HandGesture);
}

struct Part1Parser;

impl LogParser for Part1Parser {
    fn parse_log(&self, log: &str) -> (HandGesture, HandGesture) {
        let mut splitted = log.split(' ');
        (
            HandGesture::from(splitted.next().unwrap()),
            HandGesture::from(splitted.next().unwrap()),
        )
    }
}

struct Part2Parser;

impl LogParser for Part2Parser {
    fn parse_log(&self, log: &str) -> (HandGesture, HandGesture) {
        let mut splitted = log.split(' ');
        let enemy = HandGesture::from(splitted.next().unwrap());
        let you = BattleResult::from(splitted.next().unwrap()).when_enemy(enemy.clone());
        (enemy, you)
    }
}

trait Scorer {
    fn score(&self) -> u32;
}

#[derive(Clone, Debug, PartialEq)]
enum HandGesture {
    Rock,
    Scissors,
    Paper,
}

#[derive(Debug, PartialEq)]
enum BattleResult {
    Win,
    Draw,
    Lose,
}

impl BattleResult {
    fn from(c: &str) -> BattleResult {
        match c {
            "X" => BattleResult::Lose,
            "Y" => BattleResult::Draw,
            "Z" => BattleResult::Win,
            _ => panic!("not handled"),
        }
    }

    fn when_enemy(&self, enemy: HandGesture) -> HandGesture {
        match self {
            BattleResult::Win => match enemy {
                HandGesture::Rock => HandGesture::Paper,
                HandGesture::Paper => HandGesture::Scissors,
                HandGesture::Scissors => HandGesture::Rock,
            },
            BattleResult::Draw => match enemy {
                HandGesture::Rock => HandGesture::Rock,
                HandGesture::Paper => HandGesture::Paper,
                HandGesture::Scissors => HandGesture::Scissors,
            },
            BattleResult::Lose => match enemy {
                HandGesture::Rock => HandGesture::Scissors,
                HandGesture::Paper => HandGesture::Rock,
                HandGesture::Scissors => HandGesture::Paper,
            },
        }
    }
}

impl HandGesture {
    fn from(c: &str) -> HandGesture {
        match c {
            "A" | "X" => HandGesture::Rock,
            "B" | "Y" => HandGesture::Paper,
            "C" | "Z" => HandGesture::Scissors,
            _ => panic!("not handled"),
        }
    }
}

impl Scorer for HandGesture {
    fn score(&self) -> u32 {
        match self {
            HandGesture::Rock => 1,
            HandGesture::Paper => 2,
            HandGesture::Scissors => 3,
        }
    }
}

impl Scorer for BattleResult {
    fn score(&self) -> u32 {
        match self {
            BattleResult::Lose => 0,
            BattleResult::Draw => 3,
            BattleResult::Win => 6,
        }
    }
}

fn battle(enemy: HandGesture, you: HandGesture) -> BattleResult {
    match enemy {
        HandGesture::Scissors => match you {
            HandGesture::Rock => BattleResult::Win,
            HandGesture::Scissors => BattleResult::Draw,
            HandGesture::Paper => BattleResult::Lose,
        },
        HandGesture::Rock => match you {
            HandGesture::Rock => BattleResult::Draw,
            HandGesture::Scissors => BattleResult::Lose,
            HandGesture::Paper => BattleResult::Win,
        },
        HandGesture::Paper => match you {
            HandGesture::Rock => BattleResult::Lose,
            HandGesture::Scissors => BattleResult::Win,
            HandGesture::Paper => BattleResult::Draw,
        },
    }
}

fn score_collector(scores: Vec<Box<dyn Scorer>>) -> u32 {
    scores.iter().map(|x| x.score()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rock_defeats_scissors() {
        assert_eq!(
            BattleResult::Win,
            battle(HandGesture::Scissors, HandGesture::Rock)
        );

        assert_eq!(
            BattleResult::Lose,
            battle(HandGesture::Rock, HandGesture::Scissors)
        );

        assert_eq!(
            BattleResult::Draw,
            battle(HandGesture::Rock, HandGesture::Rock)
        );
    }

    #[test]
    fn scissors_defeats_paper() {
        assert_eq!(
            BattleResult::Win,
            battle(HandGesture::Paper, HandGesture::Scissors)
        );

        assert_eq!(
            BattleResult::Lose,
            battle(HandGesture::Scissors, HandGesture::Paper)
        );

        assert_eq!(
            BattleResult::Draw,
            battle(HandGesture::Scissors, HandGesture::Scissors)
        );
    }

    #[test]
    fn paper_defeats_rock() {
        assert_eq!(
            BattleResult::Win,
            battle(HandGesture::Rock, HandGesture::Paper)
        );

        assert_eq!(
            BattleResult::Lose,
            battle(HandGesture::Paper, HandGesture::Rock)
        );

        assert_eq!(
            BattleResult::Draw,
            battle(HandGesture::Paper, HandGesture::Paper)
        );
    }

    #[test]
    fn sum_all_scores() {
        assert_eq!(1, score_collector(vec![Box::new(HandGesture::Rock)]));
        assert_eq!(2, score_collector(vec![Box::new(HandGesture::Paper)]));
        assert_eq!(3, score_collector(vec![Box::new(HandGesture::Scissors)]));

        assert_eq!(6, score_collector(vec![Box::new(BattleResult::Win)]));
        assert_eq!(3, score_collector(vec![Box::new(BattleResult::Draw)]));
        assert_eq!(0, score_collector(vec![Box::new(BattleResult::Lose)]));

        assert_eq!(
            15,
            score_collector(vec![
                Box::new(HandGesture::Rock),
                Box::new(HandGesture::Paper),
                Box::new(HandGesture::Scissors),
                Box::new(BattleResult::Win),
                Box::new(BattleResult::Lose),
                Box::new(BattleResult::Draw),
            ])
        )
    }

    #[test]
    fn part1_should_be_15() {
        const DATA: &str = "A Y\nB X\nC Z";
        assert_eq!(15, calculate_result(DATA, Box::new(Part1Parser)));
    }

    // part 2

    #[test]
    fn calculate_hand_gesture_when_enemy_choose_rock() {
        assert_eq!(
            HandGesture::Paper,
            BattleResult::Win.when_enemy(HandGesture::Rock)
        );

        assert_eq!(
            HandGesture::Rock,
            BattleResult::Draw.when_enemy(HandGesture::Rock)
        );

        assert_eq!(
            HandGesture::Scissors,
            BattleResult::Lose.when_enemy(HandGesture::Rock)
        );
    }

    #[test]
    fn calculate_hand_gesture_when_enemy_choose_paper() {
        assert_eq!(
            HandGesture::Scissors,
            BattleResult::Win.when_enemy(HandGesture::Paper)
        );

        assert_eq!(
            HandGesture::Paper,
            BattleResult::Draw.when_enemy(HandGesture::Paper)
        );

        assert_eq!(
            HandGesture::Rock,
            BattleResult::Lose.when_enemy(HandGesture::Paper)
        );
    }

    #[test]
    fn calculate_hand_gesture_when_enemy_choose_scissors() {
        assert_eq!(
            HandGesture::Rock,
            BattleResult::Win.when_enemy(HandGesture::Scissors)
        );

        assert_eq!(
            HandGesture::Scissors,
            BattleResult::Draw.when_enemy(HandGesture::Scissors)
        );

        assert_eq!(
            HandGesture::Paper,
            BattleResult::Lose.when_enemy(HandGesture::Scissors)
        );
    }

    #[test]
    fn part_2_should_be_12() {
        const DATA: &str = "A Y\nB X\nC Z";
        assert_eq!(12, calculate_result(DATA, Box::new(Part2Parser)));
    }
}
