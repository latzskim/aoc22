use std::{cell::RefCell, fs};

fn main() {
    let cargo_schema = fs::read_to_string("input_cargo.txt").unwrap();
    let cargo_moves = fs::read_to_string("input_moves.txt").unwrap();

    rearrange_cargo(&cargo_schema, &cargo_moves, Box::new(MovablePart1));
    rearrange_cargo(&cargo_schema, &cargo_moves, Box::new(MovablePart2));
}

fn rearrange_cargo(cargo_schema: &str, cargo_moves: &str, movable: Box<dyn Movable>) {
    let x = parse_cargo(cargo_schema);

    x.iter().for_each(|f| println!("{:?}", f));

    let cargo_stack: &mut Vec<RefCell<Vec<char>>> =
        &mut x.iter().map(|x| RefCell::new(x.clone())).collect();

    cargo_moves
        .lines()
        .map(parse_move)
        .for_each(|cargo_move| movable.move_cargo(cargo_stack, cargo_move));

    let tops: String = cargo_stack
        .iter()
        .map(|v| {
            let d = v.borrow();
            *d.first().unwrap()
        })
        .collect();

    println!("{:?}", tops);
}

trait Movable {
    fn move_cargo(&self, cargo_stack: &mut Vec<RefCell<Vec<char>>>, cargo_move: Vec<usize>);
}

struct MovablePart1;
impl Movable for MovablePart1 {
    fn move_cargo(&self, cargo_stack: &mut Vec<RefCell<Vec<char>>>, cargo_move: Vec<usize>) {
        let count = *cargo_move.first().unwrap();
        let from = *cargo_move.get(1).unwrap() - 1;
        let to = *cargo_move.last().unwrap() - 1;

        let mut cargo_from = cargo_stack.get(from).unwrap().borrow_mut();
        let mut cargo_to = cargo_stack.get(to).unwrap().borrow_mut();

        for _ in 0..count {
            let cargo = cargo_from.remove(0);
            cargo_to.insert(0, cargo);
        }
    }
}

struct MovablePart2;
impl Movable for MovablePart2 {
    fn move_cargo(&self, cargo_stack: &mut Vec<RefCell<Vec<char>>>, cargo_move: Vec<usize>) {
        let count = *cargo_move.first().unwrap();
        let from = *cargo_move.get(1).unwrap() - 1;
        let to = *cargo_move.last().unwrap() - 1;

        let mut cargo_from = cargo_stack.get(from).unwrap().borrow_mut();
        let mut cargo_to = cargo_stack.get(to).unwrap().borrow_mut();

        cargo_from
            .drain(0..count)
            .rev()
            .for_each(|cargo| cargo_to.insert(0, cargo));
    }
}

fn parse_move(cargo_move: &str) -> Vec<usize> {
    cargo_move
        .replace("move ", "")
        .replace("from ", "")
        .replace("to ", "")
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

fn parse_cargo(cargo_schema: &str) -> Vec<Vec<char>> {
    cargo_schema
        .lines()
        .inspect(|f| println!("{}", f))
        .fold(vec![], |mut acc, x| {
            x.chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .for_each(|(index, cargo_char)| {
                    let cargo = acc.get_mut(index);
                    if let Some(v) = cargo {
                        if cargo_char != ' ' {
                            v.push(cargo_char);
                        }
                    } else if cargo_char != ' ' {
                        acc.push(vec![cargo_char]);
                    } else {
                        acc.push(vec![]);
                    }
                });
            acc
        })
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn parse_cargo_as_vec_of_vecs() {
        let data1 = fs::read_to_string("input_cargo.txt").unwrap();

        assert_eq!(
            vec![
                vec!['P', 'L', 'M', 'N', 'W', 'V', 'B', 'H'],
                vec!['H', 'Q', 'M'],
                vec!['L', 'M', 'Q', 'F', 'G', 'B', 'D', 'N'],
                vec!['G', 'W', 'M', 'Q', 'F', 'T', 'Z'],
                vec!['P', 'H', 'T', 'M'],
                vec!['T', 'G', 'H', 'D', 'J', 'M', 'B', 'C'],
                vec!['R', 'V', 'F', 'B', 'N', 'M'],
                vec!['S', 'G', 'R', 'M', 'H', 'L', 'P'],
                vec!['N', 'C', 'B', 'D', 'P'],
            ],
            parse_cargo(&data1)
        );
    }

    #[test]
    fn parse_move_as_vec_of_numbers() {
        const DATA: &str = "move 10 from 14 to 3";

        assert_eq!(vec![10, 14, 3], parse_move(DATA));
    }
}
