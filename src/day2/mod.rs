use RoundResult::*;
use Shape::*;

#[derive(Debug, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

enum RoundResult {
    Win,
    Draw,
    Lost,
}

#[derive(Debug)]
struct Round {
    player: Shape,
    opponent: Shape,
}

impl Shape {
    fn from(shape_string: &str) -> Self {
        match shape_string {
            "A" | "X" => Rock,
            "B" | "Y" => Paper,
            _ => Scissors,
        }
    }

    fn from_srategy(opponent: &Shape, expectation: &RoundResult) -> Self {
        match (&opponent, &expectation) {
            (Rock, Win) => Paper,
            (Rock, Lost) => Scissors,
            (Paper, Win) => Scissors,
            (Paper, Lost) => Rock,
            (Scissors, Win) => Rock,
            (Scissors, Lost) => Paper,
            _ => *opponent, // Draw. Use the same shape as opponent by cloning.
        }
    }

    fn get_strategy(strategy_string: &str) -> RoundResult {
        match strategy_string {
            "X" => Lost,
            "Y" => Draw,
            _ => Win,
        }
    }
}

fn calculate_score(round: &Round) -> usize {
    let result = match (&round.player, &round.opponent) {
        (Rock, Scissors) => Win,
        (Rock, Paper) => Lost,
        (Scissors, Paper) => Win,
        (Scissors, Rock) => Lost,
        (Paper, Rock) => Win,
        (Paper, Scissors) => Lost,
        _ => Draw,
    };

    let round_score = match result {
        Win => 6,
        Draw => 3,
        Lost => 0,
    };

    let shape_score = match round.player {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    };

    // dbg!("score: {} for {:?}", round_score + shape_score, round);
    round_score + shape_score
}

fn parse_round_strategy(line: &str) -> Round {
    let strs: Vec<&str> = line.split(' ').collect();
    let opponent = Shape::from(strs[0]);

    Round {
        opponent,
        player: Shape::from_srategy(&opponent, &Shape::get_strategy(strs[1])),
    }
}

fn parse_round(line: &str) -> Round {
    let strs: Vec<&str> = line.split(' ').collect();

    Round {
        opponent: Shape::from(strs[0]),
        player: Shape::from(strs[1]),
    }
}

fn solve_part2(input: &[String]) -> usize {
    input
        .iter()
        .map(|line| parse_round_strategy(line))
        .map(|round| calculate_score(&round))
        .sum()
}

fn solve_part1(input: &[String]) -> usize {
    input
        .iter()
        .map(|line| parse_round(line))
        .map(|round| calculate_score(&round))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::fs_helper;

    #[test]
    fn test_part1_example() {
        let lines: Vec<String> = fs_helper::read_lines("./src/day2/example_2-1.txt")
            .unwrap()
            .map(|line| line.unwrap())
            .collect();
        let answer = solve_part1(&lines);
        assert_eq!(answer, 15);
    }

    #[test]
    fn test_part1_input() {
        let lines: Vec<String> = fs_helper::read_lines("./src/day2/input_2-1.txt")
            .unwrap()
            .map(|line| line.unwrap())
            .collect();

        let answer = solve_part1(&lines);
        println!("Answer of day 2-1: {}", answer);
        assert_eq!(answer, 13_484);
    }

    #[test]
    fn test_part2_example() {
        let lines: Vec<String> = fs_helper::read_lines("./src/day2/example_2-1.txt")
            .unwrap()
            .map(|line| line.unwrap())
            .collect();

        let answer = solve_part2(&lines);
        assert_eq!(answer, 12);
    }

    #[test]
    fn test_part2_input() {
        let lines: Vec<String> = fs_helper::read_lines("./src/day2/input_2-1.txt")
            .unwrap()
            .map(|line| line.unwrap())
            .collect();

        let answer = solve_part2(&lines);
        println!("Answer of day 2-2: {}", answer);
        assert_eq!(answer, 13_433);
    }
}
