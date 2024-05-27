use crate::group::cycle::Cycle;
use std::hash::Hash;
use crate::generator::generator::Generator;

pub struct PeriodicGen<T> 
where
    T: Copy+Clone+Hash+Eq+'static
{
    start: Vec<T>,
    permutations: Vec<fn(T) -> T>
}

impl<T> PeriodicGen<T> 
where
    T: Copy+Clone+Hash+Eq+'static
{
    fn new() -> Self {
        PeriodicGen {
            start: Vec::new(),
            permutations: Vec::new()
        }
    }
}

impl<T> Generator<T> for PeriodicGen<T>
where
    T: Copy+Clone+Hash+Eq+'static
{
    fn start(&self) -> Vec<T> {
        self.start
    }

    fn set_start(&self, start: &Vec<T>) {
        self.start = self.start.clone()
    }

    fn add(&self, f: fn(T) -> T) {
        self.permutations.push(f);
    }

    fn clear(&self) {
        self.permutations.clear();
    }



    fn iter(&self) -> super::generator::GenIter<T> {
        todo!()
    }
}