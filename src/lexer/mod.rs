use crate::instruction;
use std::collections::HashMap;
use regex::Regex;


#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenType {
    Opcode,
    Register,
    IntegerOperand,
    Number
}

pub type ResultToken = (TokenType, &'static str);

pub enum Token {
    Opcode(instruction::Opcode),
    Register(u8),
    IntegerOperand(i32)
}

pub struct AssemblerInstruction {
    opcode: Token,
    arg1: Option<Token>,
    arg2: Option<Token>,
    arg3: Option<Token>,
}

pub struct Grammar {
    pub rules: HashMap<String, TokenType>,
}

impl Grammar {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    pub fn add_rule(&mut self, src: &str, token_type: TokenType) {
        self.rules.insert(src.to_string(), token_type);
    }

    pub fn parse_str(&self, src: &'static str) -> Option<ResultToken> {
        for (s, token) in self.rules.iter() {
            let re = Regex::new(s.as_str()).unwrap();
            if re.is_match(src) {
                println!("Found!");
                match token {
                    TokenType::Opcode => {
                        let t = Token::Opcode(instruction::Opcode::from(src));
                        return Some((token.clone(), src))
                    },
                    TokenType::Register => {
                        return Some((token.clone(), src.split_at(1).1))
                    },
                    TokenType::IntegerOperand => {
                        return Some((token.clone(), src.split_at(1).1))
                    },
                    TokenType::Number => {
                        return Some((token.clone(), src))
                    }
                }
            }
        }
        None
    }
}

pub fn build_grammar() -> Grammar {
    let mut grammar = Grammar::new();
    grammar.add_rule(r"^[a-z]+$", TokenType::Opcode);
    grammar.add_rule(r"\$\d{1,2}", TokenType::Register);
    grammar 
}


mod tests {
    use super::*;

    #[test]
    fn test_opcode_load() {
        let grammar = build_grammar();
        assert_eq!(grammar.parse_str("load").unwrap(), (TokenType::Opcode, "load"));
        assert_eq!(grammar.parse_str("asz123"), None);
    }
}