use bimap::BiMap;
use std::hash::Hash;
use std::{collections::HashMap, ops::Mul};

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
    T: Clone+Hash+Eq
{
    type Output = Cycle<T>;

    fn mul(self, rhs: Cycle<T>) -> Self::Output {
        let same_ground = self.ground.clone();

        let mut new_map: BiMap<T, T> = BiMap::new();

        
        for g in self.ground {
            let pcyc = match rhs.map.get_by_left(&g).and_then(|f| self.map.get_by_left(f)) {
                Some(res) => res.clone(),
                None => panic!("Compositions should not be empty!")
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