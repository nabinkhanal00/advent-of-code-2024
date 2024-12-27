pub mod part1 {
    use std::{
        error::Error,
        fs::File,
        io::{self, BufRead},
    };

    use regex::Regex;
    type Combo = i64;
    type Literal = i64;
    #[derive(Debug)]
    enum Instruction {
        ADV(Combo),
        BXL(Literal),
        BST(Combo),
        JNZ(Literal),
        BXC,
        OUT(Combo),
        BDV(Combo),
        CDV(Combo),
    }
    impl Instruction {
        fn run(&self, state: &mut State) {
            match self {
                Instruction::ADV(op) => {
                    let num = state.a;
                    let den = 2_i64.pow(Self::get_combo_value(state, *op) as u32);
                    state.a = num / den;
                }
                Instruction::BXL(op) => {
                    state.b = state.b ^ *op;
                }
                Instruction::BST(op) => {
                    state.b = Self::get_combo_value(state, *op) % 8;
                }
                Instruction::JNZ(op) => {
                    if state.a != 0 {
                        state.ip = *op;
                    }
                }
                Instruction::BXC => {
                    state.b = state.b ^ state.c;
                }
                Instruction::OUT(op) => {
                    state.output.push(Self::get_combo_value(state, *op) % 8);
                }
                Instruction::BDV(op) => {
                    let num = state.a;
                    let den = 2_i64.pow(Self::get_combo_value(state, *op) as u32);
                    state.b = num / den;
                }
                Instruction::CDV(op) => {
                    let num = state.a;
                    let den = 2_i64.pow(Self::get_combo_value(state, *op) as u32);
                    state.c = num / den;
                }
            }
        }
        fn get_combo_value(state: &State, op: i64) -> i64 {
            match op {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 3,
                4 => state.a,
                5 => state.b,
                6 => state.c,
                _ => unreachable!(),
            }
        }
    }
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct State {
        a: i64,
        b: i64,
        c: i64,

        ip: i64,
        output: Vec<i64>,
    }
    #[derive(Debug)]
    struct Program {
        instructions: Vec<i64>,
        state: State,
    }
    impl Program {
        fn parse(filename: &str) -> Result<(State, Vec<i64>), Box<dyn Error>> {
            let file = File::open(filename)?;
            let reader = io::BufReader::new(file);
            let mut lines = reader.lines();
            let re = Regex::new(r"\d+")?;
            let a: i64 = re
                .find(lines.next().unwrap()?.as_str())
                .unwrap()
                .as_str()
                .parse()?;
            let b: i64 = re
                .find(lines.next().unwrap()?.as_str())
                .unwrap()
                .as_str()
                .parse()?;
            let c: i64 = re
                .find(lines.next().unwrap()?.as_str())
                .unwrap()
                .as_str()
                .parse()?;
            lines.next();

            let re = Regex::new(r"\d+")?;
            let input: Vec<i64> = re
                .find_iter(lines.next().unwrap()?.as_str())
                .filter_map(|n| n.as_str().parse::<i64>().ok())
                .collect();

            let state = State {
                a,
                b,
                c,
                ip: 0,
                output: Vec::new(),
            };
            Ok((state, input))
        }
        fn tokenize(opcode: i64, operand: i64) -> Instruction {
            let instruction;
            match opcode {
                0 => {
                    instruction = Instruction::ADV(operand);
                }
                1 => {
                    instruction = Instruction::BXL(operand);
                }
                2 => {
                    instruction = Instruction::BST(operand);
                }
                3 => {
                    instruction = Instruction::JNZ(operand);
                }
                4 => {
                    instruction = Instruction::BXC;
                }
                5 => {
                    instruction = Instruction::OUT(operand);
                }
                6 => {
                    instruction = Instruction::BDV(operand);
                }
                7 => {
                    instruction = Instruction::CDV(operand);
                }
                _ => unreachable!(),
            }
            instruction
        }
        fn compile(filename: &str) -> Result<Self, Box<dyn Error>> {
            let (state, instructions) = Self::parse(filename)?;
            Ok(Self {
                state,
                instructions,
            })
        }
        fn run(&mut self) {
            while (self.state.ip as usize + 1) < self.instructions.len() {
                let opcode = self.instructions[self.state.ip as usize];
                let operand = self.instructions[(self.state.ip + 1) as usize];
                let instruction = Self::tokenize(opcode, operand);
                self.state.ip += 2;
                instruction.run(&mut self.state);
            }
        }
    }
    pub fn solve() -> Result<(), Box<dyn Error>> {
        let mut program = Program::compile("src/day17/input.txt")?;
        program.run();
        for o in program.state.output {
            print!("{},", o);
        }
        println!();
        Ok(())
    }
}

pub mod part2 {
    use regex::Regex;
    use std::{
        error::Error,
        fs::File,
        io::{self, BufRead},
    };

    #[derive(Debug)]
    struct Program {
        instructions: Vec<i64>,
    }
    impl Program {
        fn parse(filename: &str) -> Result<Vec<i64>, Box<dyn Error>> {
            let file = File::open(filename)?;
            let reader = io::BufReader::new(file);
            let mut lines = reader.lines();
            lines.next();
            lines.next();
            lines.next();
            lines.next();

            let re = Regex::new(r"\d+")?;
            let input: Vec<i64> = re
                .find_iter(lines.next().unwrap()?.as_str())
                .filter_map(|n| n.as_str().parse::<i64>().ok())
                .collect();

            Ok(input)
        }
        fn compile(filename: &str) -> Result<Self, Box<dyn Error>> {
            let instructions = Self::parse(filename)?;
            Ok(Self { instructions })
        }
        fn output(a: usize) -> usize {
            let mut a = a;
            let mut result = 0;
            while a != 0 {
                let mut b = a % 8;
                b ^= 2;
                let c = a / 2u32.pow(b as u32) as usize;
                a /= 8;
                b ^= c;
                b ^= 7;
                result = result * 10 + (b % 8);
            }
            result
        }
        fn search(program: &String, start: usize, idx: usize) -> Option<String> {
            let p = program
                .chars()
                .enumerate()
                .filter_map(|(i, d)| if i >= idx { Some(d) } else { None })
                .collect::<String>();
            for c in start..start + 8 {
                let r = Self::output(c);
                if format!("{}", r) == *program {
                    return Some(format!("{}", c));
                }
                if r == p.parse::<usize>().unwrap() {
                    if let Some(sol) = Self::search(program, c * 8, idx - 1) {
                        return Some(sol);
                    }
                }
            }
            None
        }
    }

    pub fn solve() -> Result<(), Box<dyn Error>> {
        let program = Program::compile("src/day17/input.txt")?;
        let instructions: Vec<u8> = program
            .instructions
            .into_iter()
            .map(|x| '0' as u8 + (x as u8))
            .collect();
        let instructions = String::from_utf8(instructions)?;
        let output = Program::search(&instructions, 1, instructions.len() - 1).unwrap();
        println!("{}", output);
        Ok(())
    }
}
