use std::collections::HashMap;
use std::io::Read;

type Token = char;

struct Tokens {
    tokens: Vec<Token>,
    jump_table: HashMap<usize, usize>,
}

impl From<&str> for Tokens {
    fn from(source: &str) -> Self {
        let tokens: Vec<Token> = source.chars().filter(|c| "><+-.,[]".contains(*c)).collect();
        let jump_table = Tokens::build_jumptable(&Tokens {
            tokens: tokens.clone(),
            jump_table: HashMap::new(),
        });
        Tokens { tokens, jump_table }
    }
}

impl Tokens {
    fn print(&self) {
        for token in &self.tokens {
            print!("{}", token);
        }
        println!();
    }

    const fn len(&self) -> usize {
        self.tokens.len()
    }

    fn build_jumptable(tokens: &Self) -> HashMap<usize, usize> {
        let mut map: HashMap<usize, usize> = HashMap::new();
        let mut stack: Vec<usize> = Vec::new();
        for (i, token) in tokens.tokens.iter().enumerate() {
            match token {
                '[' => stack.push(i),
                ']' => {
                    if let Some(open_index) = stack.pop() {
                        map.insert(open_index, i);
                        map.insert(i, open_index);
                    } else {
                        panic!("Unmatched ']' at position {}", i);
                    }
                }
                _ => {}
            }
        }
        return map;
    }
}

struct VM {
    tape: Vec<u8>,
    pointer: usize,
}

impl VM {
    fn new() -> Self {
        VM {
            tape: vec![0u8; 30000],
            pointer: 0,
        }
    }

    fn execute(&mut self, tokens: &Tokens) {
        let mut instruction_pointer = 0;
        println!("executing {} tokens", tokens.len());
        while instruction_pointer < tokens.len() {
            match tokens.tokens[instruction_pointer] {
                '>' => self.pointer += 1,
                '<' => self.pointer -= 1,
                '+' => self.tape[self.pointer] += 1,
                '-' => self.tape[self.pointer] -= 1,
                '.' => {
                    print!("{}", self.tape[self.pointer] as char);
                }
                ',' => {
                    let mut input = [0u8; 1];
                    std::io::stdin().read_exact(&mut input).unwrap();
                    self.tape[self.pointer] = input[0];
                }
                '[' => {
                    if self.tape[self.pointer] == 0 {
                        instruction_pointer = tokens.jump_table[&instruction_pointer];
                    }
                }
                ']' => {
                    if self.tape[self.pointer] != 0 {
                        instruction_pointer = tokens.jump_table[&instruction_pointer];
                    }
                }
                _ => {}
            }
            instruction_pointer += 1;
        }
    }
}

fn main() {
    let source = "++++++++[>++++++++>++++++++++++<<-]>+>+>++++++++++>++++++++++++++++++++++++++[<<<.>.>.<<+>+>>-]";
    let tokens: Tokens = source.into();
    tokens.print();

    let mut vm = VM::new();
    vm.execute(&tokens);
}
