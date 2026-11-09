use holo_core::{Node, NodeType};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum QasmError {
    #[error("Lexer error at position {0}: {1}")]
    Lexer(usize, String),
    #[error("Parse error at line {0}: {1}")]
    Parse(usize, String),
    #[error("Semantic error: {0}")]
    Semantic(String),
}

/// Token types for OpenQASM 3 subset
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    OpenQASM,
    Version(String),
    Include,
    StringLit(String),
    Qubit,
    Bit,
    Identifier(String),
    Integer(i64),
    Float(f64),
    LBracket,
    RBracket,
    LParen,
    RParen,
    Semicolon,
    Comma,
    Arrow,
    Plus,
    Minus,
    Star,
    Slash,
    Eof,
}

/// Lexer for OpenQASM 3 subset
pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    fn skip_whitespace_and_comments(&mut self) {
        let bytes = self.input.as_bytes();
        while self.pos < bytes.len() {
            let b = bytes[self.pos];
            if b.is_ascii_whitespace() {
                self.pos += 1;
            } else if b == b'/' && self.pos + 1 < bytes.len() && bytes[self.pos + 1] == b'/' {
                self.pos += 2;
                while self.pos < bytes.len() && bytes[self.pos] != b'\n' {
                    self.pos += 1;
                }
            } else if b == b'/' && self.pos + 1 < bytes.len() && bytes[self.pos + 1] == b'*' {
                self.pos += 2;
                while self.pos + 1 < bytes.len()
                    && !(bytes[self.pos] == b'*' && bytes[self.pos + 1] == b'/')
                {
                    self.pos += 1;
                }
                if self.pos + 1 < bytes.len() {
                    self.pos += 2;
                }
            } else {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> Result<Token, QasmError> {
        self.skip_whitespace_and_comments();
        if self.pos >= self.input.len() {
            return Ok(Token::Eof);
        }

        let bytes = self.input.as_bytes();
        let b = bytes[self.pos];

        match b {
            b'[' => {
                self.pos += 1;
                Ok(Token::LBracket)
            }
            b']' => {
                self.pos += 1;
                Ok(Token::RBracket)
            }
            b'(' => {
                self.pos += 1;
                Ok(Token::LParen)
            }
            b')' => {
                self.pos += 1;
                Ok(Token::RParen)
            }
            b';' => {
                self.pos += 1;
                Ok(Token::Semicolon)
            }
            b',' => {
                self.pos += 1;
                Ok(Token::Comma)
            }
            b'+' => {
                self.pos += 1;
                Ok(Token::Plus)
            }
            b'-' => {
                if self.pos + 1 < bytes.len() && bytes[self.pos + 1] == b'>' {
                    self.pos += 2;
                    Ok(Token::Arrow)
                } else {
                    self.pos += 1;
                    Ok(Token::Minus)
                }
            }
            b'*' => {
                self.pos += 1;
                Ok(Token::Star)
            }
            b'/' => {
                self.pos += 1;
                Ok(Token::Slash)
            }
            b'"' => {
                self.pos += 1;
                let start = self.pos;
                while self.pos < bytes.len() && bytes[self.pos] != b'"' {
                    self.pos += 1;
                }
                if self.pos >= bytes.len() {
                    return Err(QasmError::Lexer(start, "Unterminated string literal".into()));
                }
                let s = &self.input[start..self.pos];
                self.pos += 1;
                Ok(Token::StringLit(s.to_string()))
            }
            _ if b.is_ascii_digit() => {
                let start = self.pos;
                let mut has_dot = false;
                while self.pos < bytes.len()
                    && (bytes[self.pos].is_ascii_digit() || bytes[self.pos] == b'.')
                {
                    if bytes[self.pos] == b'.' {
                        has_dot = true;
                    }
                    self.pos += 1;
                }
                let num_str = &self.input[start..self.pos];
                if has_dot {
                    let val: f64 = num_str
                        .parse()
                        .map_err(|e: std::num::ParseFloatError| QasmError::Lexer(start, e.to_string()))?;
                    Ok(Token::Float(val))
                } else {
                    let val: i64 = num_str
                        .parse()
                        .map_err(|e: std::num::ParseIntError| QasmError::Lexer(start, e.to_string()))?;
                    Ok(Token::Integer(val))
                }
            }
            _ if b.is_ascii_alphabetic() || b == b'_' => {
                let start = self.pos;
                while self.pos < bytes.len()
                    && (bytes[self.pos].is_ascii_alphanumeric() || bytes[self.pos] == b'_')
                {
                    self.pos += 1;
                }
                let ident = &self.input[start..self.pos];
                match ident {
                    "OPENQASM" => Ok(Token::OpenQASM),
                    "include" => Ok(Token::Include),
                    "qubit" => Ok(Token::Qubit),
                    "bit" => Ok(Token::Bit),
                    _ => Ok(Token::Identifier(ident.to_string())),
                }
            }
            _ => Err(QasmError::Lexer(
                self.pos,
                format!("Unexpected character: {}", b as char),
            )),
        }
    }
}

/// AST Representation of OpenQASM 3 Program
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QasmAst {
    pub version: Option<String>,
    pub includes: Vec<String>,
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Statement {
    QubitDecl {
        name: String,
        size: Option<usize>,
    },
    BitDecl {
        name: String,
        size: Option<usize>,
    },
    GateCall {
        gate: String,
        params: Vec<Expr>,
        qubits: Vec<QubitRef>,
    },
    Measure {
        qubit: QubitRef,
        target: BitRef,
    },
    Barrier {
        qubits: Vec<QubitRef>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QubitRef {
    pub register: String,
    pub index: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BitRef {
    pub register: String,
    pub index: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Expr {
    Number { value: f64 },
    Pi,
    BinaryOp { op: String, lhs: Box<Expr>, rhs: Box<Expr> },
    Ident { name: String },
}

/// Recursive-descent parser for OpenQASM 3
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Token,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Result<Self, QasmError> {
        let mut lexer = Lexer::new(input);
        let current = lexer.next_token()?;
        Ok(Self { lexer, current })
    }

    fn advance(&mut self) -> Result<Token, QasmError> {
        let next = self.lexer.next_token()?;
        Ok(std::mem::replace(&mut self.current, next))
    }

    fn expect(&mut self, expected: Token) -> Result<(), QasmError> {
        if self.current == expected {
            self.advance()?;
            Ok(())
        } else {
            Err(QasmError::Parse(
                0,
                format!("Expected {:?}, got {:?}", expected, self.current),
            ))
        }
    }

    pub fn parse_program(&mut self) -> Result<QasmAst, QasmError> {
        let mut version = None;
        let mut includes = Vec::new();
        let mut statements = Vec::new();

        if self.current == Token::OpenQASM {
            self.advance()?;
            match &self.current {
                Token::Float(v) => {
                    version = Some(v.to_string());
                    self.advance()?;
                }
                Token::Integer(v) => {
                    version = Some(v.to_string());
                    self.advance()?;
                }
                _ => return Err(QasmError::Parse(0, "Expected version after OPENQASM".into())),
            }
            self.expect(Token::Semicolon)?;
        }

        while self.current != Token::Eof {
            match &self.current {
                Token::Include => {
                    self.advance()?;
                    if let Token::StringLit(inc) = &self.current {
                        includes.push(inc.clone());
                        self.advance()?;
                        self.expect(Token::Semicolon)?;
                    } else {
                        return Err(QasmError::Parse(0, "Expected string after include".into()));
                    }
                }
                Token::Qubit => {
                    self.advance()?;
                    let size = if self.current == Token::LBracket {
                        self.advance()?;
                        if let Token::Integer(sz) = self.current {
                            self.advance()?;
                            self.expect(Token::RBracket)?;
                            Some(sz as usize)
                        } else {
                            return Err(QasmError::Parse(0, "Expected qubit register size".into()));
                        }
                    } else {
                        None
                    };
                    if let Token::Identifier(name) = &self.current {
                        let name_str = name.clone();
                        self.advance()?;
                        self.expect(Token::Semicolon)?;
                        statements.push(Statement::QubitDecl {
                            name: name_str,
                            size,
                        });
                    } else {
                        return Err(QasmError::Parse(0, "Expected qubit register name".into()));
                    }
                }
                Token::Bit => {
                    self.advance()?;
                    let size = if self.current == Token::LBracket {
                        self.advance()?;
                        if let Token::Integer(sz) = self.current {
                            self.advance()?;
                            self.expect(Token::RBracket)?;
                            Some(sz as usize)
                        } else {
                            return Err(QasmError::Parse(0, "Expected bit register size".into()));
                        }
                    } else {
                        None
                    };
                    if let Token::Identifier(name) = &self.current {
                        let name_str = name.clone();
                        self.advance()?;
                        self.expect(Token::Semicolon)?;
                        statements.push(Statement::BitDecl {
                            name: name_str,
                            size,
                        });
                    } else {
                        return Err(QasmError::Parse(0, "Expected bit register name".into()));
                    }
                }
                Token::Identifier(gate_name) => {
                    let gate = gate_name.clone();
                    self.advance()?;

                    if gate == "barrier" {
                        let qubits = self.parse_qubit_list()?;
                        self.expect(Token::Semicolon)?;
                        statements.push(Statement::Barrier { qubits });
                    } else if gate == "measure" {
                        let qref = self.parse_qubit_ref()?;
                        self.expect(Token::Arrow)?;
                        let bref = self.parse_bit_ref()?;
                        self.expect(Token::Semicolon)?;
                        statements.push(Statement::Measure {
                            qubit: qref,
                            target: bref,
                        });
                    } else {
                        let mut params = Vec::new();
                        if self.current == Token::LParen {
                            self.advance()?;
                            while self.current != Token::RParen && self.current != Token::Eof {
                                params.push(self.parse_expr()?);
                                if self.current == Token::Comma {
                                    self.advance()?;
                                }
                            }
                            self.expect(Token::RParen)?;
                        }
                        let qubits = self.parse_qubit_list()?;
                        self.expect(Token::Semicolon)?;
                        statements.push(Statement::GateCall {
                            gate: gate.to_lowercase(),
                            params,
                            qubits,
                        });
                    }
                }
                _ => {
                    return Err(QasmError::Parse(
                        0,
                        format!("Unexpected token: {:?}", self.current),
                    ));
                }
            }
        }

        Ok(QasmAst {
            version,
            includes,
            statements,
        })
    }

    fn parse_qubit_list(&mut self) -> Result<Vec<QubitRef>, QasmError> {
        let mut list = Vec::new();
        loop {
            list.push(self.parse_qubit_ref()?);
            if self.current == Token::Comma {
                self.advance()?;
            } else {
                break;
            }
        }
        Ok(list)
    }

    fn parse_qubit_ref(&mut self) -> Result<QubitRef, QasmError> {
        if let Token::Identifier(reg) = &self.current {
            let reg_name = reg.clone();
            self.advance()?;
            let index = if self.current == Token::LBracket {
                self.advance()?;
                if let Token::Integer(idx) = self.current {
                    self.advance()?;
                    self.expect(Token::RBracket)?;
                    Some(idx as usize)
                } else {
                    return Err(QasmError::Parse(0, "Expected integer qubit index".into()));
                }
            } else {
                None
            };
            Ok(QubitRef {
                register: reg_name,
                index,
            })
        } else {
            Err(QasmError::Parse(0, format!("Expected qubit identifier, got {:?}", self.current)))
        }
    }

    fn parse_bit_ref(&mut self) -> Result<BitRef, QasmError> {
        if let Token::Identifier(reg) = &self.current {
            let reg_name = reg.clone();
            self.advance()?;
            let index = if self.current == Token::LBracket {
                self.advance()?;
                if let Token::Integer(idx) = self.current {
                    self.advance()?;
                    self.expect(Token::RBracket)?;
                    Some(idx as usize)
                } else {
                    return Err(QasmError::Parse(0, "Expected integer bit index".into()));
                }
            } else {
                None
            };
            Ok(BitRef {
                register: reg_name,
                index,
            })
        } else {
            Err(QasmError::Parse(0, format!("Expected bit identifier, got {:?}", self.current)))
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, QasmError> {
        let mut left = match &self.current {
            Token::Integer(i) => {
                let v = *i as f64;
                self.advance()?;
                Expr::Number { value: v }
            }
            Token::Float(f) => {
                let v = *f;
                self.advance()?;
                Expr::Number { value: v }
            }
            Token::Identifier(s) => {
                let id = s.clone();
                self.advance()?;
                if id == "pi" {
                    Expr::Pi
                } else {
                    Expr::Ident { name: id }
                }
            }
            Token::Minus => {
                self.advance()?;
                let inner = self.parse_expr()?;
                Expr::BinaryOp {
                    op: "*".into(),
                    lhs: Box::new(Expr::Number { value: -1.0 }),
                    rhs: Box::new(inner),
                }
            }
            _ => return Err(QasmError::Parse(0, format!("Unexpected token in expression: {:?}", self.current))),
        };

        while self.current == Token::Slash || self.current == Token::Star || self.current == Token::Plus || self.current == Token::Minus {
            let op = match self.current {
                Token::Slash => "/",
                Token::Star => "*",
                Token::Plus => "+",
                Token::Minus => "-",
                _ => unreachable!(),
            };
            self.advance()?;
            let right = match &self.current {
                Token::Integer(i) => {
                    let v = *i as f64;
                    self.advance()?;
                    Expr::Number { value: v }
                }
                Token::Float(f) => {
                    let v = *f;
                    self.advance()?;
                    Expr::Number { value: v }
                }
                Token::Identifier(s) => {
                    let id = s.clone();
                    self.advance()?;
                    if id == "pi" {
                        Expr::Pi
                    } else {
                        Expr::Ident { name: id }
                    }
                }
                _ => return Err(QasmError::Parse(0, "Expected RHS expression".into())),
            };
            left = Expr::BinaryOp {
                op: op.to_string(),
                lhs: Box::new(left),
                rhs: Box::new(right),
            };
        }

        Ok(left)
    }
}

/// Convert parsed OpenQASM 3 into a content-addressed AST node
pub fn parse_qasm_to_ast_node(source_code: &str, source_node_id: &str) -> Result<Node, QasmError> {
    let mut parser = Parser::new(source_code)?;
    let ast = parser.parse_program()?;
    let payload = serde_json::to_value(&ast).map_err(|e| QasmError::Semantic(e.to_string()))?;

    Ok(Node::new(
        NodeType::Ast,
        payload,
        vec![source_node_id.to_string()],
        vec![],
        serde_json::json!({
            "ast_format": "holospaces_ast_v1"
        }),
        vec!["ast".to_string(), "openqasm3".to_string()],
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_grover_2qubit() {
        let qasm = r#"
OPENQASM 3.0;
include "stdgates.inc";

qubit[2] q;

// Superposition
h q[0];
h q[1];

// Oracle for |11>
cz q[0], q[1];

// Diffusion
h q[0];
h q[1];
x q[0];
x q[1];
cz q[0], q[1];
x q[0];
x q[1];
h q[0];
h q[1];
"#;
        let mut parser = Parser::new(qasm).unwrap();
        let ast = parser.parse_program().unwrap();
        assert_eq!(ast.version, Some("3".to_string()));
        assert_eq!(ast.includes, vec!["stdgates.inc"]);
        assert_eq!(ast.statements.len(), 13);
    }
}
