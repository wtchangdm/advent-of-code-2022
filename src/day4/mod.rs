#[derive(Debug)]
struct Pair {
    x: (u8, u8),
    y: (u8, u8),
}

impl Pair {
    fn from(pair_text: &str) -> Self {
        let strs: Vec<_> = pair_text.split(',').collect();
        let x: Vec<u8> = strs[0]
            .split('-')
            .map(|c| c.parse::<u8>().unwrap())
            .collect();
        let y: Vec<u8> = strs[1]
            .split('-')
            .map(|c| c.parse::<u8>().unwrap())
            .collect();

        Self {
            x: (x[0], x[1]),
            y: (y[0], y[1]),
        }
    }

    /// check if there are fully overlapped area
    /// e.g., 2-8,3-7 is considered as overlapped, while 2-4,4-5 is not
    fn is_overlapped(&self) -> bool {
        let x_len = self.x.1 - self.x.0;
        let y_len = self.y.1 - self.y.0;

        if x_len >= y_len {
            return self.x.1 >= self.y.1 && self.x.0 <= self.y.0;
        }

        self.y.1 >= self.x.1 && self.y.0 <= self.x.0
    }

    /// check if there are overlapped area (boundries are included)
    /// e.g., 2-5,4-6 and 4-6,2-5 and 2-4,4-5 are considered as overlapped
    fn is_overlapped_with_boundry(&self) -> bool {
        (self.x.0 <= self.y.0 && self.x.1 >= self.y.0)
            || (self.y.0 <= self.x.0 && self.y.1 >= self.x.0)
    }
}

fn solve_part2(input: &[String]) -> usize {
    input
        .iter()
        .map(|line| Pair::from(line))
        .filter(|pair| pair.is_overlapped_with_boundry())
        .count()
}

fn solve_part1(input: &[String]) -> usize {
    input
        .iter()
        .map(|line| Pair::from(line))
        .filter(|pair| pair.is_overlapped())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::fs_helper;

    #[test]
    fn test_part1_example() {
        let lines: Vec<String> = fs_helper::read_lines("./src/day4/example_4-1.txt")
            .unwrap()
            .map(|line| line.unwrap())
            .collect();
        let answer = solve_part1(&lines);
        assert_eq!(answer, 2);
    }

    #[test]
    fn test_part1_input() {
        let lines: Vec<String> = fs_helper::read_lines("./src/day4/input_4-1.txt")
            .unwrap()
            .map(|line| line.unwrap())
            .collect();

        let answer = solve_part1(&lines);
        println!("Answer of day 4-1: {}", answer);
        assert_eq!(solve_part1(&lines), 441);
    }

    #[test]
    fn test_part2_example() {
        let lines: Vec<String> = fs_helper::read_lines("./src/day4/example_4-1.txt")
            .unwrap()
            .map(|line| line.unwrap())
            .collect();

        let answer = solve_part2(&lines);
        assert_eq!(answer, 4);
    }

    #[test]
    fn test_part2_input() {
        let lines: Vec<String> = fs_helper::read_lines("./src/day4/input_4-1.txt")
            .unwrap()
            .map(|line| line.unwrap())
            .collect();

        let answer = solve_part2(&lines);
        println!("Answer of day 4-2: {}", answer);
        assert_eq!(solve_part2(&lines), 861);
    }
}
