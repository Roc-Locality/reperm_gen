use std::hash::Hash;
use std::fmt::Debug;
use bimap::{BiHashMap, BiMap};

use crate::group::group::Group;
use crate::group::cycle::Cycle;

pub fn sym(n_size: i32) -> SymmetricGroup<i32> {
    let ground: Vec<i32> = (1..=n_size).collect();
    SymmetricGroup {
        n: n_size as usize,
        ground: ground,
    }
}

#[derive(Default)]
pub struct SymmetricGroup<V> {
    /// the base size of a permutation the order should be n!
    n: usize,
    ground: Vec<V>,
}
impl<V> SymmetricGroup<V> 
where
    V: Clone+Copy+Hash+Eq+PartialEq+Debug
{
    pub fn new(n_size: usize, g: Vec<V>) -> SymmetricGroup<V> {
        SymmetricGroup {
            n: n_size,
            ground: g,
        }
    }

    pub fn get_ground(&self) -> Vec<V> {
        self.ground.clone()
    }

    ///Shorthand for making new cycles with respect to a symmetric group.
    pub fn create(&self, map: BiHashMap<V, V>) -> Cycle<V> {
        Cycle::new(map, self.get_ground())
    }

    pub fn create_vec(&self, lis: Vec<Vec<V>>) -> Cycle<V> {
        Cycle::from(lis, self.get_ground())
    } 
    
}
#[allow(dead_code)]
impl<V> Group<Cycle<V>> for SymmetricGroup<V> 
where 
    V: Clone+Copy+Hash+Eq+PartialEq+Debug
{
    ///Note this operation is from right to left (like normal functions)
    ///We will also assume that if Cycle<V> is implemented correctly, it is properly associative.
    fn op(&self, a: Cycle<V>, b: Cycle<V>) -> Cycle<V> {
        a * b
    }

    fn identity(&self) -> Cycle<V> {
        let mut map = BiMap::new();
        for g in self.ground.iter() {
            map.insert(*g, *g);
        }
        Cycle::new(map, self.ground.clone())
    }

    fn inverse(&self, e: Cycle<V>) -> Cycle<V> {
        e.inverse()
    }

    /// The order of a symmetric group means the size.
    /// Computing the size is too hard, so we will just use the following formula instead.
    fn order(&self) -> i32 {
        (1..=self.n as i32).product()
    }

    /// In Symmetric Groups, we can generate every single n! possible permutations by only combining and adding adjacent transpositions, ie (a_i, a_{i + 1}) for all i.
    /// We can also perhaps use a different class of generators, but this is fine for now.
    fn get_generator(&self) -> Vec<Cycle<V>> {
        self.ground.windows(2).map(|window| {
            let prev = window[0];
            let curr = window[1];
            let mut map = BiMap::new();
            map.insert(prev, curr);
            map.insert(curr, prev);
            Cycle::new(map, self.ground.clone())
        }).collect()
    }
}

mod tests {
    #[allow(unused_imports)]
    use crate::group::{cycle::Cycle, group::Group, symmetric::SymmetricGroup};

    #[test]
    fn test_symmetric_3() { 
        let ground: Vec<i32> = vec![1, 2, 3];
        let group: SymmetricGroup<i32> = SymmetricGroup::new((&ground).len(), ground.clone());
        let symmetric_set = group.get_set();
        
        debug_assert_eq!(symmetric_set.contains(&Cycle::from(vec![vec![1, 2, 3]], ground.clone())), true);
        debug_assert_eq!(symmetric_set.contains(&Cycle::from(vec![vec![1, 2]], ground.clone())), true);
        debug_assert_eq!(symmetric_set.contains(&Cycle::from(vec![vec![1, 3]], ground.clone())), true);
        debug_assert_eq!(symmetric_set.contains(&Cycle::from(vec![vec![2, 3]], ground.clone())), true);
        debug_assert_eq!(symmetric_set.contains(&Cycle::from(vec![vec![3, 2, 1]], ground.clone())), true);
        debug_assert_eq!(symmetric_set.contains(&Cycle::from(vec![vec![3, 1, 2]], ground.clone())), true);
    }
    #[test]
    fn test_symmetric_5() { 
        let ground: Vec<i32> = vec![1, 2, 3, 4, 5];
        let group: SymmetricGroup<i32> = SymmetricGroup::new((&ground).len(), ground.clone());
        let symmetric_set = group.get_set();
        debug_assert_eq!(group.order(), symmetric_set.len() as i32);
        debug_assert_eq!(group.order(), 120);
    }

    #[test]
    fn test_symmetric_8() { 
        let ground: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let group: SymmetricGroup<i32> = SymmetricGroup::new((&ground).len(), ground.clone());
        let symmetric_set = group.get_set();
        debug_assert_eq!(40320, symmetric_set.len() as i32);
    }
}