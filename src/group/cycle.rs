use bimap::BiMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Mul;
use crate::bimap;

///For permutations
#[derive(Clone, Debug)]
pub struct Cycle<T> 
where 
    T: Clone+Hash+Eq+'static
{
    ground: Vec<T>,
    ///this is the max cycle size
    n: usize,
    /// The main way to define a bimap
    map: BiMap<T, T>,
}

impl<T> Cycle<T> 
where 
    T: Clone+Eq+Hash+'static
{
    pub fn new(map: BiMap<T, T>, ground: Vec<T>) -> Self {
        Cycle { 
            ground: ground,
            n: map.len(),
            map: map
        }
    }

    pub fn from(vec: Vec<Vec<T>>, ground: Vec<T>) -> Self {
        let mut set: HashSet<T> = HashSet::from_iter(ground.iter().cloned());
        let mut map = BiMap::new();
        for cycle in vec.into_iter() {
            for window in cycle.windows(2) {
                let prev = window[0].clone();
                let curr = window[1].clone();
                
                set.remove(&prev);
                map.insert(prev, curr);
                
            }
            if let (Some(last), Some(first)) = (cycle.last(), cycle.first()) {
                set.remove(&last);
                map.insert(last.clone(), first.clone());
            }

            // Handle unchanged elements:
            for e in set.iter() {
                map.insert(e.clone(), e.clone());
            }
        }
        
        Cycle {
            ground: ground.clone(),
            n: map.len(),
            map: map
        }
    }

    
    pub fn inverse(&self) -> Self {
        let mut co = BiMap::new();
        for g in self.ground.clone() {
            co.insert(g.clone(), self.map.get_by_left(&g).unwrap().clone());
        }
        Cycle::new(co, self.ground.clone())
    }

    pub fn eval(&self, i: T) -> T {
        match self.map.get_by_left(&i) {
            Some(res) => res.clone(),
            _ => i // if it doesn't match, we will make an assumption here that it will just return the same thing
        }
    }
}

impl<T> Mul for Cycle<T> 
where 
    T: Clone+Hash+Eq+Debug
{
    type Output = Cycle<T>;

    fn mul(self, rhs: Cycle<T>) -> Self::Output {
        let same_ground = self.ground.clone();

        let mut new_map: BiMap<T, T> = BiMap::new();

        
        for g in self.ground {
            let pcyc = match rhs.map.get_by_left(&g).and_then(|f| self.map.get_by_left(f)) {
                Some(res) => res.clone(),
                None => panic!("Compositions should not be empty! query: {:?} rhs: {:?}, lhs: {:?}", g, rhs.map, self.map)
            };
            new_map.insert(g, pcyc);
        }
        
        Cycle::new(new_map, same_ground)
    }
}


impl<'a, T: Copy> PartialEq for Cycle<T> 
where 
    T: Clone+Hash+Eq
{
    fn eq(&self, other: &Self) -> bool {
        self.map.eq(&other.map)
    }
}   

/// Tests, mainly associative


#[cfg(test)]
mod tests {
    use crate::group::cycle::Cycle;

    #[test]
    fn construction1() {
        let ground = vec![1, 2, 3, 4, 5];
        let f = Cycle::from(vec![vec![1, 3], vec![4, 5]], ground.clone());
        let g = Cycle::from(vec![vec![1, 2, 5], vec![3, 4]], ground.clone());
        // f = 1 3 4 5 
        //     3 1 5 4
        // g = 1 2 5 3 4
        //     2 5 1 4 3
        debug_assert_eq!(f.map, crate::bimap![1 => 3, 3 => 1, 4 => 5, 5 => 4, 2 => 2]);
        debug_assert_eq!(g.map, crate::bimap![1 => 2, 2 => 5, 5 => 1, 3 => 4, 4 => 3]);
    }
    #[test]
    fn mul_eq1() {
        let ground = vec![1, 2, 3, 4, 5];
        let f = Cycle::from(vec![vec![1, 3], vec![4, 5]], ground.clone());
        let g = Cycle::from(vec![vec![1, 2, 5], vec![3, 4]], ground.clone());
        let fg: Cycle<_> = Cycle::from(vec![vec![1, 2, 4], vec![3, 5]], ground.clone());
        
        debug_assert_eq!((f * g).map, fg.map);
        
    }
}