use super::State;
use std::{
    convert::{AsMut, AsRef},
    mem::size_of,
    ops::{Index, IndexMut},
};

#[derive(Clone, Copy)]
pub struct Bool {
    vars: [bool; size_of::<usize>() * 8],
    var_count: usize,
}

impl State for Bool {
    type Output = bool;

    fn new(var_count: usize) -> Self {
        Bool {
            var_count,
            vars: [false; size_of::<usize>() * 8],
        }
    }

    fn iterate(&mut self, iteration: usize) {
        for index in 0..self.var_count {
            self.vars[index] = (iteration & (1 << index)) != 0;
        }
    }

    fn var_at(&self, idx: usize) -> &Self::Output {
        &self[idx]
    }

    fn var_count(&self) -> usize {
        self.var_count
    }
}

impl Index<usize> for Bool {
    type Output = bool;

    fn index(&self, index: usize) -> &bool {
        if index < self.var_count {
            &self.vars[index]
        } else {
            panic!("index out of Bounds");
        }
    }
}

impl IndexMut<usize> for Bool {
    fn index_mut(&mut self, index: usize) -> &mut bool {
        if index < self.var_count {
            &mut self.vars[index]
        } else {
            panic!("index out of Bounds");
        }
    }
}

impl AsRef<[bool]> for Bool {
    fn as_ref(&self) -> &[bool] {
        &self.vars[0..self.var_count]
    }
}

impl AsMut<[bool]> for Bool {
    fn as_mut(&mut self) -> &mut [bool] {
        &mut self.vars[0..self.var_count]
    }
}
