use std::marker::PhantomData;
use std::collections::HashSet;
use crate::group::group::{Group, GroupIter};
use crate::group::cycle::Cycle;

#[derive(Default)]
pub struct SymmetricGroup<V> {
    /// the base size of a permutation the order should be n!
    n: usize,   
    phantom: PhantomData<V>
}

impl<V> Group<Cycle<V>> for SymmetricGroup<Cycle<V>> {
    fn get_set(&self) -> HashSet<Cycle<V>> {
        todo!()
    }

    fn op(&self, a: Cycle<V>, b: Cycle<V>) -> Cycle<V> {
        todo!()
    }

    fn identity(&self) -> Cycle<V> {
        todo!()
    }

    fn inverse(&self, e: Cycle<V>) -> Cycle<V> {
        todo!()
    }

    fn order(&self) -> usize {
        todo!()
    }
    
    fn get_generator(&self) -> Vec<Cycle<V>> {
        todo!()
    }
}