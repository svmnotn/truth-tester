use super::{Lexer, Token, TokenLiterals, Tokens};
use alloc::vec::Vec;

/// Boolean Expression Parser
#[derive(Debug)]
pub struct Parser<'i> {
    lexer: Lexer<'i>,
}

impl<'i> Parser<'i> {
    pub fn parse(input: &'i str) -> Self {
        Self {
            lexer: Lexer::lex(input),
        }
    }

    pub fn parse_with_literals(input: &'i str, literals: TokenLiterals) -> Self {
        Self {
            lexer: Lexer::lex_with_literals(input, literals),
        }
    }

    pub fn shunting_yard(&mut self) -> Tokens<'i> {
        use Token::*;
        let mut toks: Vec<Token> = Vec::new();
        let mut stack: Vec<Token> = Vec::new();

        while let Some(t) = self.lexer.next() {
            match t {
                // Values can go directly to the output
                v @ Var(..) | v @ Literal(..) => toks.push(v),
                // Parens mess mostly with the stack
                LParen => stack.push(LParen),
                RParen => {
                    let mut found = false;
                    while let Some(tok) = stack.pop() {
                        if tok == LParen {
                            found = true;
                            break;
                        }

                        toks.push(tok);
                    }

                    if found == false {
                        panic!("Mismached parens!");
                    }
                }
                t => {
                    while let Some(tok) = stack.pop() {
                        if tok.precedence() > t.precedence() {
                            toks.push(tok);
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
            toks.push(t);
        }

        Tokens::new(toks, self.lexer.var_map())
    }
}
