enum NFA {
    CharNode(char),
    OrNode(Box<NFA>, Box<NFA>),
    StarNode(Box<NFA>),
    SeqNode(Box<NFA>, Box<NFA>),
}

impl NFA {
    fn compile(pattern: &str) -> NFA {
        let mut stack: Vec<NFA> = Vec::new();

        // Example:
        // a(b|c)* => abc|*.
        // -----------------
        // a -> [CharNode(a)]
        // a -> [CharNode(a)]
        // b -> [CharNode(a), CharNode(b)]
        // c -> [CharNode(a), CharNode(b), CharNode(c)]
        // | -> [CharNode(a), OrNode(CharNode(b), CharNode(c))]
        // * -> [CharNode(a), StarNode(OrNode(CharNode(b), CharNode(c)))]
        // . -> [SeqNode(CharNode(a), StarNode(OrNode(CharNode(b), CharNode(c))))]

        for c in pattern.chars() {
            match c {
                'A'...'z' => stack.push(NFA::CharNode(c)),
                '|' => {
                    let f = stack.pop().expect("Bad Pattern: Not Enough Values in Stack for Or");
                    let s = stack.pop().expect("Bad Pattern: Not Enough Values in Stack for Or");
                    stack.push(NFA::OrNode(Box::new(f), Box::new(s)));
                },
                '*' => {
                    let f = stack.pop().expect("Bad Pattern: Not Enough Values in Stack for Star");
                    stack.push(NFA::StarNode(Box::new(f)));
                },
                '.' => {
                    let f = stack.pop().expect("Bad Pattern: Not Enough Values in Stack for Link");
                    let s = stack.pop().expect("Bad Pattern: Not Enough Values in Stack for Link");
                    stack.push(NFA::SeqNode(Box::new(f), Box::new(s)));
                }
                _ => panic!("Bad Pattern: Unrecognized Character"),
            }
        }

        assert_eq!(stack.len(), 1, "Bad Pattern: Faulty stack");

        stack.pop().expect("Bad Pattern")
    }

    fn check(&self, string: &str) -> &str {
        match self {
            NFA::CharNode(c) => {
                match string.chars().next() {
                    None => false,
                    Some(p) => *c == p, // TODO why is c a ref here?
                }
            },
            NFA::OrNode(l, r) => {
                l.check(&string) || r.check(&string)
            },
            NFA::StarNode(n) => {
                let 
                while n.check(&string[1..]) {}
                true, i
            },
            NFA::SeqNode(l, r) => {
                l.check(&string[1..]) && r.check(&string[1..])
            }
        }
    }

    fn check(&self, )
}

fn main() {
    let exp = NFA::compile("abc|*.");

    assert!(exp.check("abccbbbccbb"));
    assert!(!exp.check("abcd"));
}