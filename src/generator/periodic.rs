use std::hash::Hash;
use crate::generator::generator::Generator;

pub struct PeriodicGen<T> 
where
    T: Copy+Clone+Hash+Eq+'static
{
    start: Vec<T>,
    permutations: Vec<Box<dyn Fn(T) -> T>>
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

impl<'a, T> Generator<'a, T> for PeriodicGen<T>
where
    T: Copy+Clone+Hash+Eq+'static
{
    fn start(&self) -> Vec<T> {
        self.start.clone()
    }

    fn set_start(&mut self, start: &Vec<T>) {
        self.start = start.clone()
    }

    fn add(&mut self, f: Box<dyn Fn(T) -> T>) {
        self.permutations.push(f);
    }

    fn clear(&mut self) {
        self.permutations.clear();
    }



    fn iter(&'a self) -> Box<dyn Iterator<Item = Vec<T>> + 'a> {
        Box::new(PeriodicGenIter::new(self))
    }
}

pub struct PeriodicGenIter<'a, T> 
where
    T: Copy+Clone+Hash+Eq+'static
{
    curr: Vec<T>,
    index_state: usize,
    generator: &'a PeriodicGen<T>,
}

impl<'a, T> PeriodicGenIter<'a, T> 
where
    T: Copy+Clone+Hash+Eq+'static
{
    fn new(periodic_generator: &'a PeriodicGen<T>) -> Self {
        PeriodicGenIter {
            curr: periodic_generator.start(),
            index_state: 0,
            generator: periodic_generator
        }
    }
}

impl<'a, T> Iterator for PeriodicGenIter<'a, T> 
where 
    T: Copy+Clone+Hash+Eq+'static
{
    type Item = Vec<T>;
    
    fn next(&mut self) -> Option<Self::Item> {
        let funcs = &self.generator.permutations;

        let next_vec = self.curr
            .iter()
            .map(|x| funcs[self.index_state](*x))
            .collect();
        self.index_state = (self.index_state + 1) % (funcs.len());
        let old = self.curr.clone();
        self.curr = next_vec;

        Some(old)
    }
}