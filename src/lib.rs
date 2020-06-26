use std::iter;
use std::collections::HashSet;

/// Regular expression instructions. Some instructions include integer offsets to other states.
/// These offsets correspond to instructions at other indices in the 'code' and can simply be added
/// to the current index to find the correct state.
#[derive(Debug, Copy, Clone)]
enum Instruction {
    /// A character literal.
    Literal(char),
    /// A split in the 'code'. The offset points to the start of the other split.
    Split(usize),
    /// A jump in the 'code'. The offset points to the instruction to jump to.
    Jump(isize),
    /// A match instruction. If the 'code' ends up at the end of the string in one of these states
    /// the string is a match.
    Match,
}

/// A regular expression object to represent one regular expression
pub struct Regex {
    instructions: Vec<Instruction>,
}

impl Regex {
    /// Compiles a regular expression in 'Reverse Polish Form'.
    pub fn compile(pattern: &str) -> Regex {
        let mut stack: Vec<Vec<Instruction>> = Vec::new();

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
    
    /// Expands a pointer to an instruction to a list of possible literal or match instructions.
    ///
    /// This is useful because when the matching algorithm finds a state like a jump it doesn't
    /// know which characters that jump can actually match. Following these jumps recursively gives
    /// us a list of possible literals that could be matched.
    fn expand_state(&self, state: usize) -> HashSet<usize> {
        match self.instructions[state] {
            Instruction::Literal(_) => [state].iter().cloned().collect(),
            Instruction::Split(to) => {
                let l = self.expand_state(state + 1);
                let r = self.expand_state(state + to);

                l.union(&r).cloned().collect()
            }
            Instruction::Jump(to) => self.expand_state((state as isize + to) as usize),
            Instruction::Match => [state].iter().cloned().collect(),
        }
    }
    
    /// Check if the given text is a match on this regular expression
    pub fn is_match(&self, text: &str) -> bool {
        let states: HashSet<usize> = [0].iter().cloned().collect();

        let end_states: HashSet<usize> = text.chars().fold(states, |acc, c| {
            acc.into_iter().map(|i| match self.instructions[i] {
                Instruction::Literal(e) if e == c => self.expand_state(i + 1), 
                _ => HashSet::new(),
            }).flatten().collect()
        });

        end_states.contains(&(self.instructions.len() - 1))
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
