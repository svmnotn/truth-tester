mod boolean;
pub use boolean::Bool;

pub trait State {
    type Output;

    fn new(var_count: usize) -> Self;
    fn iterate(&mut self, iter: usize);
    fn var_at(&self, idx: usize) -> &Self::Output;
    fn var_count(&self) -> usize;
}
