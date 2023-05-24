use std::collections::HashMap;

fn get_distinct_consecutive_messages(input: &str, distinct_num: usize) -> usize {
    let mut map: HashMap<char, usize> = HashMap::with_capacity(26);
    let mut start: usize = 0;
    let mut answer: usize = 0;

    for (i, char) in input.chars().enumerate() {
        if let Some(idx) = map.get(&char) {
            let new_start = idx + 1;
            if new_start > start {
                start = new_start;
            }
        }

        map.insert(char, i);

        if i - start + 1 == distinct_num {
            answer = i + 1;
            break;
        }
    }

    answer
}

fn solve_part2(input: &str) -> usize {
    get_distinct_consecutive_messages(input, 14)
}

fn solve_part1(input: &str) -> usize {
    get_distinct_consecutive_messages(input, 4)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::fs_helper;

    #[test]
    fn test_part1_example() {
        let lines: Vec<String> = fs_helper::read_lines("./src/day6/example_6-1.txt")
            .unwrap()
            .map(|line| line.unwrap())
            .collect();
        let answer = solve_part1(&lines[0]);
        assert_eq!(answer, 7);
    }

    #[test]
    fn test_part1_input() {
        let lines: Vec<String> = fs_helper::read_lines("./src/day6/input_6-1.txt")
            .unwrap()
            .map(|line| line.unwrap())
            .collect();

        let answer = solve_part1(&lines[0]);
        println!("Answer of day 6-1: {}", answer);
        assert_eq!(answer, 1_542);
    }

    #[test]
    fn test_part2_example() {
        let lines: Vec<String> = fs_helper::read_lines("./src/day6/example_6-1.txt")
            .unwrap()
            .map(|line| line.unwrap())
            .collect();

        let answer = solve_part2(&lines[0]);
        assert_eq!(answer, 19);
    }

    #[test]
    fn test_part2_input() {
        let lines: Vec<String> = fs_helper::read_lines("./src/day6/input_6-1.txt")
            .unwrap()
            .map(|line| line.unwrap())
            .collect();

        let answer = solve_part2(&lines[0]);
        println!("Answer of day 6-2: {}", answer);
        assert_eq!(solve_part2(&lines[0]), 3_153);
    }
}
