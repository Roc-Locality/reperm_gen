use std::hash::Hash;
use std::collections::HashSet;
use bimap::BiMap;

use crate::group::group::Group;
use crate::group::cycle::Cycle;

#[derive(Default)]
pub struct SymmetricGroup<V> {
    /// the base size of a permutation the order should be n!
    n: i32,
    ground: Vec<V>,
}
impl<V> SymmetricGroup<V> 
where
    V: Clone
{
    pub fn new(n: i32, ground: Vec<V>) -> SymmetricGroup<V> {
        SymmetricGroup {
            n: n,
            ground: ground,
        }
    }
    pub fn get_ground(&self) -> Vec<V> {
        self.ground.clone()
    }
}
#[allow(dead_code)]
impl<'a, V> Group<Cycle<V>> for SymmetricGroup<V> 
where 
    V: Clone+Copy+Hash+Eq+PartialEq
{
    fn get_set(&self) -> HashSet<Cycle<V>> {
        todo!()
    }

    ///Note this operation is from right to left (like normal functions)
    fn op(&self, a: Cycle<V>, b: Cycle<V>) -> Cycle<V> {
        a * b
    }

    fn identity(&self) -> Cycle<V> {
        Cycle::new(BiMap::new(), self.ground.clone())
    }

    fn inverse(&self, e: Cycle<V>) -> Cycle<V> {
        e.inverse()
    }

    fn order(&self) -> i32 {
        (1..=self.n).product()
    }

    /// In Symmetric Groups, we can generate every single n! possible permutations by only combining and adding adjacent transpositions, ie (a_i, a_{i + 1}) for all i.
    fn get_generator(&self) -> Vec<Cycle<V>> {
        let mut adj_transpositions = Vec::new();
        for window in self.ground.windows(2) {
            let prev = window[0].clone();
            let curr = window[1].clone();
            let mut map = BiMap::new();
            map.insert(prev, curr);
            let adj_t = Cycle::new(map, self.ground.clone());
            adj_transpositions.push(adj_t);
        }

        adj_transpositions
    }
}

