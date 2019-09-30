use crate::{
    eval::{Expression, State},
    parsing::{Token::*, Tokens},
};
use alloc::vec::Vec;

impl<'a> Expression for Tokens<'a> {
    fn eval<S: State>(&self, state: S) -> bool {
        let mut stack: Vec<bool> = Vec::new();
        // turn the expression into a single value
        for t in self.toks.iter() {
            match *t {
                // Values can go directly to the stack
                Var(_, v) => stack.push(state.var_at(v)),
                Literal(v) => stack.push(v),
                // Operations
                Not => {
                    if let Some(v) = stack.pop() {
                        stack.push(!v);
                    } else {
                        panic!("Ran NOT without a Variable!")
                    }
                }
                And => match (stack.pop(), stack.pop()) {
                    (Some(b), Some(a)) => stack.push(a && b),
                    _ => panic!("Ran AND without enough variables"),
                },
                Xor => match (stack.pop(), stack.pop()) {
                    (Some(b), Some(a)) => stack.push(a ^ b),
                    _ => panic!("Ran XOR without enough variables"),
                },
                Or => match (stack.pop(), stack.pop()) {
                    (Some(b), Some(a)) => stack.push(a || b),
                    _ => panic!("Ran OR without enough variables"),
                },
                Implication => match (stack.pop(), stack.pop()) {
                    (Some(b), Some(a)) => stack.push(!a || b),
                    _ => panic!("Ran IMPLICATION without enough variables"),
                },
                Equality => match (stack.pop(), stack.pop()) {
                    (Some(b), Some(a)) => stack.push(a == b),
                    _ => panic!("Ran EQUALS without enough variables"),
                },
                LParen => unreachable!("LParen in Final Experssion"),
                RParen => unreachable!("RParen in Final Experssion"),
            }
        }

        if stack.len() != 1 {
            panic!("The expression did not resove to a single value!");
        }

        stack.pop().unwrap()
    }
}
