use crate::tester::{Expression, State, Tester};
use rayon::{
    iter::{IntoParallelIterator, ParallelIterator},
    range::Iter,
};

impl<E: Expression + Send + Sync> Tester<E> {
    fn iterations_par(&self) -> Iter<usize> {
        self.iterations().into_par_iter()
    }

    pub fn is_true_par(&self) -> bool {
        self.failures_par().any(|_| true) == false
    }

    pub fn is_false_par(&self) -> bool {
        self.successes_par().any(|_| true) == false
    }

    pub fn successes_par(&self) -> impl ParallelIterator<Item = impl State> + '_ {
        self.eval_par()
            .filter_map(|(s, v)| if v == true { Some(s) } else { None })
    }

    pub fn failures_par(&self) -> impl ParallelIterator<Item = impl State> + '_ {
        self.eval_par()
            .filter_map(|(s, v)| if v == false { Some(s) } else { None })
    }

    pub fn eval_par(&self) -> impl ParallelIterator<Item = (impl State, bool)> + '_ {
        self.iterations_par()
            .map(move |iter| (iter, self.expr.eval(iter)))
    }
}
