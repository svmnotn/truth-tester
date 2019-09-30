use crate::eval::{Expression, State, Tester};

impl<E: Expression> Tester<E> {
    pub fn is_true(&self) -> bool {
        self.failures().any(|_| true) == false
    }

    pub fn is_false(&self) -> bool {
        self.successes().any(|_| true) == false
    }

    pub fn successes<'a>(&'a self) -> impl Iterator<Item = impl State> + '_ {
        self.eval()
            .filter_map(|(s, v)| if v == true { Some(s) } else { None })
    }

    pub fn failures<'a>(&'a self) -> impl Iterator<Item = impl State> + '_ {
        self.eval()
            .filter_map(|(s, v)| if v == false { Some(s) } else { None })
    }

    pub fn eval(&self) -> impl Iterator<Item = (impl State, bool)> + '_ {
        self.iterations()
            .map(move |iter| (iter, self.expr.eval(iter)))
    }
}
