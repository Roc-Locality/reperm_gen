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
    /// data = [1, 2, 3,], which is a cycle of size 3
    data: Vec<T>,
    /// This is to make calculations easier.
    map: HashMap<T, T>,
}

impl<T> Cycle<T> 
where 
    T: Clone+Eq+Hash+'static
{
    pub fn new(data: Vec<T>, ground: Vec<T>) -> Self {
        let mut map: HashMap<T, T> = HashMap::new();
        
        for window in data.windows(2) {
            let prev = window[0].clone();
            let curr = window[1].clone();
            map.insert(prev, curr);
        }
        if let (Some(last), Some(first)) = (data.last(), data.first()) {
            map.insert(last.clone(), first.clone());
        }
        Cycle { 
            data: data.to_vec(),
            ground: ground,
            n: data.len(),
            map: map
        }
    }

    
    pub fn inverse(&self) -> Self {
        let mut co = self.data.to_vec();
        co.reverse();
        Cycle::new(co, self.ground.clone())
    }

    pub fn eval(&self, i: T) -> T {
        match self.map.get(&i) {
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

        let mut new_cycle: Vec<T> = Vec::new();

        let mut cyc: T = self.ground[0].clone();
        while !new_cycle.contains(&cyc) {
            let pcyc = match rhs.map.get(&cyc).and_then(|f| self.map.get(f)) {
                Some(res) => res.clone(),
                None => panic!("Compositions should not be empty!")
            };

            if pcyc == cyc {
                break
            } 
            new_cycle.push(cyc.clone());
            cyc = pcyc;
        }
        
        Cycle::new(new_cycle, same_ground)
    }
}


impl<'a, T: Copy> PartialEq for Cycle<T> 
where 
    T: Clone+Hash+Eq
{
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}