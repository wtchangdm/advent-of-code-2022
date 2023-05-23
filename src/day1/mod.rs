fn solve_part2(input: &[String]) -> usize {
    let mut calories = vec![];
    let mut elf_calories = 0;

    input.iter().for_each(|cal| match cal.parse::<usize>() {
        Ok(cal) => elf_calories += cal,
        Err(_) => {
            calories.push(elf_calories);
            // reset for next elf
            elf_calories = 0;
        }
    });

    calories.push(elf_calories);
    calories.sort_by(|a, b| b.cmp(a));

    calories[0..3].iter().sum()
}

fn solve_part1(input: &[String]) -> usize {
    let mut max_calories = 0;
    let mut elf_calories = 0;

    input.iter().for_each(|cal| match cal.parse::<usize>() {
        Ok(cal) => elf_calories += cal,
        Err(_) => {
            if elf_calories > max_calories {
                max_calories = elf_calories;
            }
            // reset for next elf
            elf_calories = 0
        }
    });

    if elf_calories > max_calories {
        max_calories = elf_calories;
    }

    max_calories
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::fs_helper;

    #[test]
    fn test_part1_example() {
        let lines: Vec<String> = fs_helper::read_lines("./src/day1/example_1-1.txt")
            .unwrap()
            .map(|line| line.unwrap())
            .collect();
        let answer = solve_part1(&lines);
        assert_eq!(answer, 24_000);
    }

    #[test]
    fn test_part1_input() {
        let lines: Vec<String> = fs_helper::read_lines("./src/day1/input_1-1.txt")
            .unwrap()
            .map(|line| line.unwrap())
            .collect();

        let answer = solve_part1(&lines);
        println!("Answer of day 1-1: {}", answer);
        assert_eq!(solve_part1(&lines), 66_186);
    }

    #[test]
    fn test_part2_example() {
        let lines: Vec<String> = fs_helper::read_lines("./src/day1/example_1-1.txt")
            .unwrap()
            .map(|line| line.unwrap())
            .collect();

        let answer = solve_part2(&lines);
        assert_eq!(answer, 45_000);
    }

    #[test]
    fn test_part2_input() {
        let lines: Vec<String> = fs_helper::read_lines("./src/day1/input_1-1.txt")
            .unwrap()
            .map(|line| line.unwrap())
            .collect();

        let answer = solve_part2(&lines);
        println!("Answer of day 1-2: {}", answer);
        assert_eq!(solve_part2(&lines), 196_804);
    }
}
