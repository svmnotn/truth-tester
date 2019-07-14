use super::lexer::Lexer;
use super::tokens::{Tokens, Token, TokenLiterals};
use alloc::vec::Vec;

/// Boolean Expression Parser
#[derive(Debug)]
pub struct Parser<'t, 'l, 'i: 't> {
    lexer: Lexer<'t, 'l, 'i>,
}

impl<'t, 'i: 't> Parser<'t, 'static, 'i> {
    /// Create a new Parser with the
    /// Default `TokenLiterals`
    pub fn new(input: &'i str) -> Self {
        Self {
            lexer: Lexer::new(input),
        }
    }
}

impl<'t, 'l, 'i: 't> Parser<'t, 'l, 'i> {
    /// Create a new Parser with the given
    /// `TokenLiterals`
    pub fn new_with_literals(input: &'i str, literals: TokenLiterals<'l>) -> Self {
        Self {
            lexer: Lexer::new_with_literals(input, literals),
        }
    }

    /// Run the Shunting Yard algorithm on the input
    pub fn shunting_yard(&mut self) -> Tokens<'t> {
        use Token::*;
        let mut out: Vec<Token> = Vec::new();
        let mut stack: Vec<Token> = Vec::new();
        let mut var_count = 0;

        while let Some(t) = self.lexer.next() {
            match t {
                // Values can go directly to the output
                Var(s, v) => out.push(Var(s, v)),
                Literal(v) => out.push(Literal(v)),
                // Parens mess mostly with the stack
                LParen => stack.push(LParen),
                RParen => {
                    let mut found = false;
                    while let Some(tok) = stack.pop() {
                        if tok == LParen {
                            found = true;
                            break;
                        }

                        out.push(tok);
                    }

                    if found == false {
                        panic!("Mismached parens!");
                    }
                }
                EOF(v) => {
                    var_count = v;
                    break;
                }
                t => {
                    while let Some(tok) = stack.pop() {
                        if tok.precedence() > t.precedence() {
                            out.push(tok);
                        } else {
                            stack.push(tok);
                            break;
                        }
                    }
                    stack.push(t);
                }
            }
        }

        while let Some(t) = stack.pop() {
            if t == LParen || t == RParen {
                panic!("Mismached Parens!");
            }
            out.push(t);
        }

        out.push(EOF(var_count));
        out
    }
}
