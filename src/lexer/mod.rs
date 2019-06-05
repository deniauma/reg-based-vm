use crate::instruction;
use regex::Regex;


#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenType {
    Opcode,
    Register,
    IntegerOperand,
}

impl From<Token> for TokenType {
    fn from(v: Token) -> Self {
        match v {
            Token::Opcode(_op) => return TokenType::Opcode,
            Token::Register(_r) => return TokenType::Register,
            Token::IntegerOperand(_) => return TokenType::IntegerOperand
        }
    }
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


#[derive(Debug, PartialEq, Copy, Clone)]
pub struct AssemblerInstruction {
    opcode: Token,
    arg1: Option<Token>,
    arg2: Option<Token>,
    arg3: Option<Token>,
}


#[derive(Debug, PartialEq, Copy, Clone)]
pub struct AssemblerInstructionRule {
    opcode: instruction::Opcode,
    arg1: Option<TokenType>,
    arg2: Option<TokenType>,
    arg3: Option<TokenType>,
}

impl AssemblerInstructionRule {
    pub fn new(op: instruction::Opcode, arg1: Option<TokenType>, arg2: Option<TokenType>, arg3: Option<TokenType>) -> Self {
        Self {
            opcode: op,
            arg1: arg1,
            arg2: arg2,
            arg3: arg3,
        }
    }

    pub fn is_match(&self, inst: AssemblerInstruction) -> bool {
        // test the opcode
        match inst.opcode {
            Token::Opcode(opc) => {
                if opc != self.opcode {
                    return false
                }
            },
            _ => return false
        };
        
        // test the arg1 type 
        if !Self::compare_token(inst.arg1, self.arg1) {
            return false
        }

        // test the arg2 type 
        if !Self::compare_token(inst.arg2, self.arg2) {
            return false
        }

        // test the arg3 type 
        if !Self::compare_token(inst.arg3, self.arg3) {
            return false
        }

        true
    }

    fn compare_token(token: Option<Token>, token_type: Option<TokenType>) -> bool {
        if token.is_none() != token_type.is_none() {
            return false
        }
        if token.is_none() {
            return true
        }
        let a = TokenType::from(token.unwrap());
        let b = token_type.unwrap();
        return a == b
    }
}


#[derive(Debug)]
pub struct Grammar {
    pub terminal_rules: Vec<TokenTypeRegex>,
    pub instruction_rules: Vec<AssemblerInstructionRule>
}

impl Grammar {
    pub fn new() -> Self {
        Self {
            terminal_rules: vec!(),
            instruction_rules: vec!()
        }
    }

    pub fn add_rule(&mut self, src: &str, token_type: TokenType) {
        self.terminal_rules.push(TokenTypeRegex::new(token_type, src));
    }

    pub fn add_intruction_rule(&mut self, rule: AssemblerInstructionRule) {
        self.instruction_rules.push(rule);
    }
}


#[derive(Debug)]
pub struct Lexer {
    grammar: Grammar
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            grammar: build_grammar()
        }
    }

    pub fn match_instruction(&self, inst: AssemblerInstruction) -> bool {
        for rule in &self.grammar.instruction_rules {
            if rule.is_match(inst) {
                return true
            }
        }
        false
    }

    pub fn parse_instruction(&self, inst: &str) -> Result<AssemblerInstruction, String> {
        let args: Vec<&str> = inst.split(" ").collect();
        let mut tokens: Vec<Token> = vec!();
        if args.len() > 4 {
            return Err(format!("Invalid instrcution, too many arguments (for '{}')", inst))
        }
        if args.len() == 0 {
            match self.parse_str(inst) {
                Ok(t) => {
                    tokens.push(t);
                },
                Err(e) => return Err(format!("No matching instruction for '{}' ({})", inst, e))
            }
        }
        else {
            for arg in &args {
                match self.parse_str(arg) {
                    Ok(t) => {
                        tokens.push(t);
                    },
                    Err(e) => return Err(format!("No matching instruction for '{}' ({})", inst, e))
                }
            }
        }
        let opcode = *tokens.get(0).unwrap();
        let arg1 = match tokens.get(1) {
            Some(&t) => Some(t),
            None => None
        };
        let arg2 = match tokens.get(2) {
            Some(&t) => Some(t),
            None => None
        };
        let arg3 = match tokens.get(3) {
            Some(&t) => Some(t),
            None => None
        };
        Ok(AssemblerInstruction {
            opcode: opcode,
            arg1: arg1,
            arg2: arg2,
            arg3: arg3
        })
        
    }

    pub fn parse_str(&self, src: &str) -> Result<Token, String> {
        for t in &self.grammar.terminal_rules {
            if t.regex.is_match(src) {
                match t.token_type {
                    TokenType::Opcode => {
                        let opcode = t.regex.captures(src).unwrap().name("op").unwrap().as_str();
                        let op = Token::Opcode(instruction::Opcode::from(opcode));
                        return Ok(op)
                    },
                    TokenType::Register => {
                        let n: u8 = t.regex.captures(src).unwrap().name("reg").unwrap().as_str().parse().unwrap();
                        return Ok(Token::Register(n))
                    },
                    TokenType::IntegerOperand => {
                        let i: i32 = t.regex.captures(src).unwrap().name("intop").unwrap().as_str().parse().unwrap();
                        return Ok(Token::IntegerOperand(i))
                    },
                }
            }
        }
        Err(format!("No matching token for '{}'", src))
    }
}

pub fn build_grammar() -> Grammar {
    let mut grammar = Grammar::new();
    grammar.add_rule(r"(?P<op>[a-z]+)", TokenType::Opcode);
    grammar.add_rule(r"\$(?P<reg>\d{1,2})", TokenType::Register);
    grammar.add_rule(r"\#(?P<intop>\d+)", TokenType::IntegerOperand);
    grammar.add_intruction_rule(AssemblerInstructionRule::new(instruction::Opcode::LOAD, Some(TokenType::Register), Some(TokenType::IntegerOperand), None));
    grammar 
}


mod tests {
    use super::*;

    #[test]
    fn test_opcode_load() {
        let lex = Lexer::new();
        assert_eq!(lex.parse_str("load"), Ok(Token::Opcode(instruction::Opcode::LOAD)));
        assert!(lex.parse_str("123").is_err());
    }

    #[test]
    fn test_register() {
        let lex = Lexer::new();
        assert_eq!(lex.parse_str("$1"), Ok(Token::Register(1)));
        assert!(lex.parse_str("$").is_err());
    }

    #[test]
    fn test_integer_operand() {
        let lex = Lexer::new();
        assert_eq!(lex.parse_str("#100"), Ok(Token::IntegerOperand(100)));
        assert!(lex.parse_str("#").is_err());
    }

    #[test]
    fn test_load_instruction() {
        let lex = Lexer::new();
        assert_eq!(lex.parse_instruction("load $1 #100"), Ok(AssemblerInstruction {
            opcode: Token::Opcode(instruction::Opcode::LOAD),
            arg1: Some(Token::Register(1)),
            arg2: Some(Token::IntegerOperand(100)),
            arg3: None
        }));
        assert!(lex.parse_instruction("load load $2 $1 #100").is_err());
    }

    #[test]
    fn test_rule_load() {
        let lex = Lexer::new();
        let inst = lex.parse_instruction("load $1 #100").unwrap();
        assert!(lex.match_instruction(inst));
    }
}