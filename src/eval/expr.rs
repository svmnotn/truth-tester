use crate::eval::State;

pub trait Expression {
    fn eval<S: State>(&self, state: S) -> bool;
}
