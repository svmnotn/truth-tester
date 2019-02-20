use crate::state::State;

pub struct Tester<S, E>
where
    S: State,
    E: Fn(&S) -> bool,
{
    state: S,
    last_iter: usize,
    expr: E,
}

impl<S, E> Tester<S, E>
where
    S: State,
    E: Fn(&S) -> bool,
{
    pub fn new(var_count: usize, expr: E) -> Tester<S, E> {
        Tester {
            state: State::new(var_count),
            last_iter: 0,
            expr,
        }
    }

    pub fn reset(&mut self) {
        self.last_iter = 0;
    }

    #[allow(clippy::bool_comparison)]
    pub fn test_sucess(&mut self) -> bool {
        while let Some(v) = self.test() {
            if v == true {
                return true;
            }
        }
        false
    }

    #[allow(clippy::bool_comparison)]
    pub fn test_fail(&mut self) -> bool {
        while let Some(v) = self.test() {
            if v == false {
                return true;
            }
        }
        false
    }

    fn test(&mut self) -> Option<bool> {
        if self.last_iter >= (1 << self.state.var_count()) {
            None
        } else {
            self.state.iterate(self.last_iter);
            self.last_iter += 1;
            Some((self.expr)(&self.state))
        }
    }
}
