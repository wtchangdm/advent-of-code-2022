use std::collections::HashSet;

/// Sum the priority of given chars.
///
/// - Lowercase item types a through z have priorities 1 through 26.
/// - Uppercase item types A through Z have priorities 27 through 52.
fn calculate_priority(chars: &[char]) -> usize {
    chars
        .iter()
        .map(|c| {
            match c.is_uppercase() {
                // 'A' is 65 in ASCII table, subtracting 38 to make it 27.
                true => *c as usize - 38,
                // 'a' is 97 in ASCII table, subtracting 96 to make it 1.
                false => *c as usize - 96,
            }
        })
        .sum()
}

fn find_duplicates(rucksacks: &[&str]) -> Vec<char> {
    rucksacks
        .iter()
        .map(|rustsack| rustsack.chars().collect::<HashSet<_>>())
        .reduce(|acc, x| {
            acc.intersection(&x)
                .map(|x| x.to_owned())
                .collect::<HashSet<_>>()
        })
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>()
}

fn solve_part2(input: &[String]) -> usize {
    let mut groups = vec![];
    let mut group = vec![];

    for (i, line) in input.iter().enumerate() {
        group.push(line.as_str());
        if i > 0 && (i + 1) % 3 == 0 {
            groups.push(group);
            group = vec![];
        }
    }

    groups
        .into_iter()
        .map(|group| find_duplicates(&group[..]))
        .map(|chars| calculate_priority(&chars))
        .sum()
}

fn solve_part1(input: &[String]) -> usize {
    input
        .iter()
        .map(|rucksack| {
            find_duplicates(&[
                &rucksack[0..rucksack.len() / 2],
                &rucksack[rucksack.len() / 2..],
            ])
        })
        .map(|chars| calculate_priority(&chars))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::fs_helper;

    #[test]
    fn test_part1_example() {
        let lines: Vec<String> = fs_helper::read_lines("./src/day3/example_3-1.txt")
            .unwrap()
            .map(|line| line.unwrap())
            .collect();
        let answer = solve_part1(&lines);
        assert_eq!(answer, 157);
    }

    #[test]
    fn test_part1_input() {
        let lines: Vec<String> = fs_helper::read_lines("./src/day3/input_3-1.txt")
            .unwrap()
            .map(|line| line.unwrap())
            .collect();

        let answer = solve_part1(&lines);
        println!("Answer of day 3-1: {}", answer);
        assert_eq!(solve_part1(&lines), 7_691);
    }

    #[test]
    fn test_part2_example() {
        let lines: Vec<String> = fs_helper::read_lines("./src/day3/example_3-1.txt")
            .unwrap()
            .map(|line| line.unwrap())
            .collect();

        let answer = solve_part2(&lines);
        assert_eq!(answer, 70);
    }

    #[test]
    fn test_part2_input() {
        let lines: Vec<String> = fs_helper::read_lines("./src/day3/input_3-1.txt")
            .unwrap()
            .map(|line| line.unwrap())
            .collect();

        let answer = solve_part2(&lines);
        println!("Answer of day 3-2: {}", answer);
        assert_eq!(solve_part2(&lines), 2_508);
    }
}
