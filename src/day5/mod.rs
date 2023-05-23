struct Instruction {
    num: usize,
    from: usize,
    to: usize,
}

struct Cargo(char);

fn parse_instructions(strings: &[String]) -> Vec<Instruction> {
    strings
        .iter()
        .map(|line| {
            let instructions = line
                .replace("move ", "")
                .replace("from ", "")
                .replace("to ", "")
                .split(' ')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            Instruction {
                num: instructions[0],
                from: instructions[1],
                to: instructions[2],
            }
        })
        .collect::<Vec<_>>()
}

fn parse_cargo_stacks(strings: &[String]) -> Vec<Vec<Cargo>> {
    let num_stacks = strings[0].len() / 3;
    let mut stacks: Vec<Vec<Cargo>> = (0..num_stacks).map(|_| Vec::<Cargo>::new()).collect();

    strings.iter().for_each(|line| {
        line.chars()
            .enumerate()
            // cargo sits at position (0-indexed) 1, 5, 9, ...
            .filter(|(i, _)| i != &0 && (i - 1) % 4 == 0)
            .enumerate()
            .for_each(|(i, (_, char))| {
                if !char.is_whitespace() {
                    stacks[i].insert(0, Cargo(char));
                }
            })
    });

    stacks
}

fn parse(input: &[String]) -> (Vec<Vec<Cargo>>, Vec<Instruction>) {
    let mut cargo_scanner_end = 0;
    for (i, line) in input.iter().enumerate() {
        if !line.starts_with(" 1") {
            // the line indicates the number of stack
            continue;
        }
        cargo_scanner_end = i;
        break;
    }

    let cargo_stacks = parse_cargo_stacks(&input[0..cargo_scanner_end]);
    let instructions = parse_instructions(&input[cargo_scanner_end + 2..]);

    (cargo_stacks, instructions)
}

fn solve_part2(input: &[String]) -> String {
    let (mut cargo_stacks, instructions) = parse(input);

    for instruction in instructions {
        let from = instruction.from - 1;
        let to = instruction.to - 1;
        let mut queue = Vec::with_capacity(instruction.num);
        for _ in 0..instruction.num {
            let cargo = cargo_stacks[from].pop().unwrap();
            queue.push(cargo);
        }

        for _ in 0..instruction.num {
            let cargo = queue.pop().unwrap();
            cargo_stacks[to].push(cargo);
        }
    }

    cargo_stacks
        .iter()
        // it's possible some stack positions have 0 cargo on it
        .filter(|s| !s.is_empty())
        .map(|s| s.last().unwrap().0)
        .collect::<String>()
}

fn solve_part1(input: &[String]) -> String {
    let (mut cargo_stacks, instructions) = parse(input);

    for instruction in instructions {
        let from = instruction.from - 1;
        let to = instruction.to - 1;
        for _ in 0..instruction.num {
            let cargo = cargo_stacks[from].pop().unwrap();
            cargo_stacks[to].push(cargo);
        }
    }

    cargo_stacks
        .iter()
        // it's possible some stack positions have 0 cargo on it
        .filter(|s| !s.is_empty())
        .map(|s| s.last().unwrap().0)
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::fs_helper;

    #[test]
    fn test_part1_example() {
        let lines: Vec<String> = fs_helper::read_lines("./src/day5/example_5-1.txt")
            .unwrap()
            .map(|line| line.unwrap())
            .collect();
        let answer = solve_part1(&lines);
        assert_eq!(answer, "CMZ");
    }

    #[test]
    fn test_part1_input() {
        let lines: Vec<String> = fs_helper::read_lines("./src/day5/input_5-1.txt")
            .unwrap()
            .map(|line| line.unwrap())
            .collect();

        let answer = solve_part1(&lines);
        println!("Answer of day 5-1: {}", answer);
        assert_eq!(solve_part1(&lines), "SHMSDGZVC");
    }

    #[test]
    fn test_part2_example() {
        let lines: Vec<String> = fs_helper::read_lines("./src/day5/example_5-1.txt")
            .unwrap()
            .map(|line| line.unwrap())
            .collect();

        let answer = solve_part2(&lines);
        assert_eq!(answer, "MCD");
    }

    #[test]
    fn test_part2_input() {
        let lines: Vec<String> = fs_helper::read_lines("./src/day5/input_5-1.txt")
            .unwrap()
            .map(|line| line.unwrap())
            .collect();

        let answer = solve_part2(&lines);
        println!("Answer of day 5-2: {}", answer);
        assert_eq!(solve_part2(&lines), "VRZGHDFBQ");
    }
}
