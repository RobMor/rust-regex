use std::iter;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Literal(char),
    Split(usize),
    Jump(isize),
    Match,
}

pub struct Regex {
    instructions: Vec<Instruction>,
}

impl Regex {
    pub fn compile(pattern: &str) -> Regex {
        let mut stack: Vec<Vec<Instruction>> = Vec::new();

        // Example:
        // a(b|c)* => abc|*.
        // -----------------
        // a -> [CharNode(a)]
        // b -> [CharNode(a), CharNode(b)]
        // c -> [CharNode(a), CharNode(b), CharNode(c)]
        // | -> [CharNode(a), OrNode(CharNode(b), CharNode(c))]
        // * -> [CharNode(a), StarNode(OrNode(CharNode(b), CharNode(c)))]
        // . -> [SeqNode(CharNode(a), StarNode(OrNode(CharNode(b), CharNode(c))))]

        for c in pattern.chars() {
            match c {
                '|' => {
                    let s = stack
                        .pop()
                        .expect("Bad Pattern: Not enough values in stack for Split");
                    let f = stack
                        .pop()
                        .expect("Bad Pattern: Not enough values in stack for Split");

                    // SPLIT
                    // f
                    // JUMP
                    // s

                    let n = iter::once(Instruction::Split(f.len() + 2))
                        .chain(f.into_iter())
                        .chain(iter::once(Instruction::Jump((s.len() + 1) as isize)))
                        .chain(s.into_iter())
                        .collect();

                    stack.push(n);
                }
                '*' => {
                    let f = stack
                        .pop()
                        .expect("Bad Pattern: Not Enough Values in Stack for Star");

                    let l = f.len();

                    // SPLIT
                    // f
                    // JUMP (back)

                    let n = iter::once(Instruction::Split(l + 2))
                        .chain(f.into_iter())
                        .chain(iter::once(Instruction::Jump(-1 * (l + 1) as isize)))
                        .collect();

                    stack.push(n);
                }
                '.' => {
                    let s = stack
                        .pop()
                        .expect("Bad Pattern: Not Enough Values in Stack for Link");
                    let f = stack
                        .pop()
                        .expect("Bad Pattern: Not Enough Values in Stack for Link");

                    // f
                    // s
                    
                    let n = f.into_iter().chain(s.into_iter()).collect();

                    stack.push(n);
                }
                c => {
                    let n = vec![Instruction::Literal(c)];

                    stack.push(n);
                }
            }
        }

        assert_eq!(stack.len(), 1, "Bad Pattern: Faulty stack");

        let mut instructions = stack.pop().unwrap();
        instructions.push(Instruction::Match);

        Regex {
            instructions: instructions,
        }
    }

    fn expand_state(&self, state: usize) -> Vec<usize> {
        match self.instructions[state] {
            Instruction::Literal(_) => {
                vec![state]
            },
            Instruction::Split(to) => {
                let mut l = self.expand_state(state + 1);
                let mut r = self.expand_state(state + to);

                l.append(&mut r);

                l
            },
            Instruction::Jump(to) => {
                self.expand_state((state as isize + to) as usize)
            },
            Instruction::Match => {
                vec![state]
            }
        }
    }

    pub fn is_match(&self, text: &str) -> bool {
        let mut current_states: Vec<usize> = vec![0];
        let mut next_states = Vec::new();
        
        for c in text.chars() {
            for index in current_states.into_iter() {
                match self.instructions[index] {
                    Instruction::Literal(e) => {
                        if e == c {
                            let mut new_states = self.expand_state(index+1);
                            next_states.append(&mut new_states);
                        }
                    },
                    Instruction::Match => (),
                    s => unreachable!("Unexpected state found in current states: {:?}", s),
                }
            }

            current_states = next_states;
            next_states = Vec::new();
        }

        for index in current_states.into_iter() {
            if let Instruction::Match = self.instructions[index] {
                return true
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn compiles() {
        Regex::compile("abc|*.");
    }

    #[test]
    fn matches() {
        let regex = Regex::compile("abc|*.");

        assert!(regex.is_match("ab"), "ab");
        assert!(regex.is_match("abc"), "abc");
        assert!(regex.is_match("abcb"), "abcb");
        assert!(!regex.is_match("abcd"), "abcd");
    }
}
