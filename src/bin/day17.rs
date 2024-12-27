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

use Instruction::*;

#[derive(Clone)]
struct Machine {
    instruction_ptr: usize,
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    output_buffer: Vec<usize>,
}

fn combo_op(op: usize, machine: &Machine) -> usize {
    match op {
        0..=3 => op,
        4 => machine.reg_a,
        5 => machine.reg_b,
        6 => machine.reg_c,
        other => panic!("Invalid combo op {other}"),
    }
}

impl Instruction {
    fn from_code(code: usize) -> Instruction {
        match code {
            0 => Adv,
            1 => Bxl,
            2 => Bst,
            3 => Jnz,
            4 => Bxc,
            5 => Out,
            6 => Bdv,
            7 => Cdv,
            other => panic!("Invalid opcode {other}"),
        }
    }
}

impl Machine {
    fn eval_one(&mut self, code: &[usize]) {
        let ins = Instruction::from_code(code[self.instruction_ptr]);
        let op = code[self.instruction_ptr + 1];
        match ins {
            Adv => {
                self.reg_a /= 2usize.pow(combo_op(op, self) as u32);
                self.instruction_ptr += 2;
            }
            Bxl => {
                self.reg_b ^= op;
                self.instruction_ptr += 2;
            }
            Bst => {
                self.reg_b = combo_op(op, self) % 8;
                self.instruction_ptr += 2;
            }
            Jnz => {
                if self.reg_a == 0 {
                    self.instruction_ptr += 2;
                } else {
                    self.instruction_ptr = op;
                }
            }
            Bxc => {
                self.reg_b ^= self.reg_c;
                self.instruction_ptr += 2;
            }
            Out => {
                self.output_buffer.push(combo_op(op, self) % 8);
                self.instruction_ptr += 2;
            }
            Bdv => {
                self.reg_b = self.reg_a / 2usize.pow(combo_op(op, self) as u32);
                self.instruction_ptr += 2;
            }
            Cdv => {
                self.reg_c = self.reg_a / 2usize.pow(combo_op(op, self) as u32);
                self.instruction_ptr += 2;
            }
        }
    }
    fn eval_program(&mut self, code: &[usize], early_term: bool) {
        while self.instruction_ptr + 1 < code.len() {
            self.eval_one(code);
            if early_term && self.output_buffer.len() > code.len() {
                return;
            } else if early_term
                && self
                    .output_buffer
                    .iter()
                    .enumerate()
                    .any(|(idx, val)| *val != code[idx])
            {
                return;
            }
        }
    }
    fn prepare_output(&self) -> String {
        self.output_buffer
            .iter()
            .map(|it| it.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }
}

fn part1(mut machine: Machine, program: &[usize]) -> String {
    machine.eval_program(program, false);
    machine.prepare_output()
}

fn part2(machine: Machine, program: &[usize]) -> usize {
    for reg_a_val in 0..usize::MAX {
        if reg_a_val % 100_000_000 == 0 {
            println!("{reg_a_val}");
        }
        let mut my_machine = machine.clone();
        my_machine.reg_a = reg_a_val;
        my_machine.eval_program(program, true);
        if &my_machine.output_buffer == program {
            return reg_a_val;
        }
    }
    panic!("Did not find a solution.");
}

fn input_machine() -> Machine {
    Machine {
        reg_a: 32916674,
        reg_b: 0,
        reg_c: 0,
        instruction_ptr: 0,
        output_buffer: vec![],
    }
}

fn input_program() -> [usize; 16] {
    [2, 4, 1, 1, 7, 5, 0, 3, 1, 4, 4, 0, 5, 5, 3, 0]
}

fn calc_b_mod_8(prev_a: usize) -> usize {
    let mut b = prev_a % 8;
    b ^= 1;
    let c = prev_a / (2usize.pow(b as u32));
    b ^= 4;
    b ^= c;
    b % 8
}

// Not actually general in the input program, since the program is hard-coded in calc_b_mod_8.
fn walk_backwards(input_program: &[usize], init_a: usize) -> usize {
    let mut a = init_a;

    for bm8 in input_program.iter().rev() {
        for a_cand in (a * 8)..usize::MAX {
            let cand_bm8 = calc_b_mod_8(a_cand);
            if cand_bm8 == *bm8 {
                a = a_cand;
                break;
            }
        }
    }
    a
}

fn main() {
    println!("Part 1: {}", part1(input_machine(), &input_program()));
    println!("Part 2: {}", walk_backwards(&input_program(), 0));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_p1() {
        let mut machine = Machine {
            reg_a: 729,
            reg_b: 0,
            reg_c: 0,
            instruction_ptr: 0,
            output_buffer: vec![],
        };
        let program = [0, 1, 5, 4, 3, 0];
        machine.eval_program(&program, false);
        assert_eq!(machine.output_buffer, vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
    }
    #[test]
    fn test_p2() {
        let machine = Machine {
            reg_a: 2024,
            reg_b: 0,
            reg_c: 0,
            instruction_ptr: 0,
            output_buffer: vec![],
        };
        let program = [0, 3, 5, 4, 3, 0];
        let reg_a_val = part2(machine, &program);
        assert_eq!(reg_a_val, 117440);
    }
}
