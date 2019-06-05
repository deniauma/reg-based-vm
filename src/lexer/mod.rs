use crate::instruction;
use std::collections::HashMap;
use regex::Regex;


#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenType {
    Opcode,
    Register,
    IntegerOperand,
}


#[derive(Debug)]
pub struct TokenTypeRegex {
    pub token_type: TokenType,
    pub regex: Regex
}

impl TokenTypeRegex {
    pub fn new(t: TokenType, re: &str) -> Self {
        Self {
            token_type: t,
            regex: Regex::new(re).unwrap()
        }
    }
}


#[derive(Debug, PartialEq, Copy, Clone)]
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
    pub terminal_rules: Vec<TokenTypeRegex>
}

impl Grammar {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
            terminal_rules: vec!()
        }
    }

    pub fn add_rule(&mut self, src: &str, token_type: TokenType) {
        self.rules.insert(src.to_string(), token_type);
        self.terminal_rules.push(TokenTypeRegex::new(token_type, src));
    }

    pub fn parse_str(&self, src: &str) -> Option<Token> {
        for t in &self.terminal_rules {
            if t.regex.is_match(src) {
                match t.token_type {
                    TokenType::Opcode => {
                        let opcode = t.regex.captures(src).unwrap().name("op").unwrap().as_str();
                        let op = Token::Opcode(instruction::Opcode::from(opcode));
                        return Some(op)
                    },
                    TokenType::Register => {
                        let n: u8 = t.regex.captures(src).unwrap().name("reg").unwrap().as_str().parse().unwrap();
                        return Some(Token::Register(n))
                    },
                    TokenType::IntegerOperand => {
                        let i: i32 = t.regex.captures(src).unwrap().name("intop").unwrap().as_str().parse().unwrap();
                        return Some(Token::IntegerOperand(i))
                    },
                }
            }
        }
        None
    }
}

pub fn build_grammar() -> Grammar {
    let mut grammar = Grammar::new();
    grammar.add_rule(r"(?P<op>[a-z]+)", TokenType::Opcode);
    grammar.add_rule(r"\$(?P<reg>\d{1,2})", TokenType::Register);
    grammar.add_rule(r"\#(?P<intop>\d+)", TokenType::IntegerOperand);
    grammar 
}


mod tests {
    use super::*;

    #[test]
    fn test_opcode_load() {
        let grammar = build_grammar();
        assert_eq!(grammar.parse_str("load"), Some(Token::Opcode(instruction::Opcode::LOAD)));
        assert_eq!(grammar.parse_str("123"), None);
    }

    #[test]
    fn test_register() {
        let grammar = build_grammar();
        assert_eq!(grammar.parse_str("$1"), Some(Token::Register(1)));
        assert_eq!(grammar.parse_str("$"), None);
    }

    #[test]
    fn test_integer_operand() {
        let grammar = build_grammar();
        assert_eq!(grammar.parse_str("#100"), Some(Token::IntegerOperand(100)));
        assert_eq!(grammar.parse_str("#"), None);
    }
}