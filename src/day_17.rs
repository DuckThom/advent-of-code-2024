use std::ops::{BitXor, Div};

pub fn execute(input: &str) {
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let (instructions, registers) = parse_input(input);

    run_program(&instructions, &registers, 10000)
        .unwrap()
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn part2(input: &str) -> usize {
    let (instructions, registers) = parse_input(input);

    let find_candidate_bits = |reg_a: usize, output: usize| -> Vec<usize> {
        let mut candidates: Vec<usize> = vec![];

        for bits in 0b000usize..=0b111usize {
            let candidate = (reg_a << 3) | bits;

            let result = run_program(
                &instructions,
                &[candidate, registers[1], registers[2], registers[3]],
                10000,
            );

            if result.is_ok() && output == *result.unwrap().first().unwrap() {
                candidates.push(candidate);
            }
        }

        candidates
    };

    instructions
        .iter()
        .rev()
        .fold(vec![0_usize], |candidates, instruction| {
            candidates
                .into_iter()
                .flat_map(|candidate| find_candidate_bits(candidate, *instruction))
                .collect::<Vec<_>>()
        })
        .into_iter()
        .min()
        .unwrap()
}

fn run_program(
    instructions: &Vec<usize>,
    initial_registers: &Registers,
    limit: usize,
) -> Result<Vec<usize>, ()> {
    let mut registers = initial_registers.clone();
    let mut output: Vec<usize> = vec![];
    let mut instruction_counter = 0;

    while registers[3] < instructions.len() && instruction_counter < limit {
        let instruction_pointer = registers[3];
        let instruction = Instruction::new(instructions[instruction_pointer]);

        let result = instruction.run(
            &mut registers,
            instructions[instruction_pointer + 1],
            &mut output,
        );
        if result.is_err() {
            break;
        }

        instruction_counter += 1;
    }

    if instruction_counter >= limit {
        return Err(());
    }

    Ok(output)
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

type Registers = [usize; 4];

impl Instruction {
    fn new(instruction: usize) -> Instruction {
        INSTRUCTIONS[instruction]
    }

    fn run(
        &self,
        registers: &mut Registers,
        operand: usize,
        output: &mut Vec<usize>,
    ) -> Result<(), ()> {
        match self {
            Instruction::Adv => {
                let divisor = 2_usize.pow(get_operand_value(registers, operand) as u32);
                if divisor == 0 {
                    return Err(());
                }

                registers[0] = registers[0].div(divisor);
                registers[3] += 2;

                Ok(())
            }
            Instruction::Bxl => {
                registers[1] = registers[1].bitxor(operand);
                registers[3] += 2;

                Ok(())
            }
            Instruction::Bst => {
                registers[1] = get_operand_value(registers, operand) % 8;
                registers[3] += 2;

                Ok(())
            }
            Instruction::Jnz => {
                if registers[0] != 0 {
                    registers[3] = operand
                } else {
                    registers[3] += 2;
                }

                Ok(())
            }
            Instruction::Bxc => {
                registers[1] = registers[1].bitxor(registers[2]);
                registers[3] += 2;

                Ok(())
            }
            Instruction::Out => {
                output.push(get_operand_value(registers, operand) % 8);
                registers[3] += 2;

                Ok(())
            }
            Instruction::Bdv => {
                let divisor = 2_usize.pow(get_operand_value(registers, operand) as u32);
                if divisor == 0 {
                    return Err(());
                }

                registers[1] = registers[0].div(divisor);
                registers[3] += 2;

                Ok(())
            }
            Instruction::Cdv => {
                let divisor = 2_usize.pow(get_operand_value(registers, operand) as u32);
                if divisor == 0 {
                    return Err(());
                }

                registers[2] = registers[0].div(divisor);
                registers[3] += 2;

                Ok(())
            }
        }
    }
}

fn get_operand_value(registers: &Registers, operand: usize) -> usize {
    match operand {
        4 => registers[0],
        5 => registers[1],
        6 => registers[2],
        _ => operand,
    }
}

const INSTRUCTIONS: [Instruction; 8] = [
    Instruction::Adv,
    Instruction::Bxl,
    Instruction::Bst,
    Instruction::Jnz,
    Instruction::Bxc,
    Instruction::Out,
    Instruction::Bdv,
    Instruction::Cdv,
];

fn parse_input(input: &str) -> (Vec<usize>, Registers) {
    let mut instructions: Vec<usize> = vec![];
    let mut registers: Registers = [0; 4];

    input.lines().for_each(|line| {
        if line.is_empty() {
            return;
        }

        if line.starts_with("Register A: ") {
            registers[0] = line.split(": ").nth(1).unwrap().parse().unwrap();
        } else if line.starts_with("Register B: ") {
            registers[1] = line.split(": ").nth(1).unwrap().parse().unwrap();
        } else if line.starts_with("Register C: ") {
            registers[2] = line.split(": ").nth(1).unwrap().parse().unwrap();
        } else if line.starts_with("Program: ") {
            instructions = line
                .split(": ")
                .nth(1)
                .unwrap()
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect();
        }
    });

    (instructions, registers)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../inputs/day_17/test");

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(TEST_INPUT), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 29328);
    }
}
