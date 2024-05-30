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
        match self.permutations.len() == 0 {
            false => Box::new(PeriodicGenIter::new(self)),
            true => panic!("permutations must have at least one element! Add to it with PeriodicGen#add")
        }
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

#[cfg(test)]
mod tests {

    use crate::{bimap, generator::generator::Generator, group::cycle::Cycle};

    use super::PeriodicGen;

    #[test]
    fn trace_abab() {
        // Make the ground set
        let ground = vec![1, 2, 3, 4, 5];
        // Make the generator
        let mut generator = PeriodicGen::new();
        generator.set_start(&ground);
        // Make your function
        let reverse_cycle = Cycle::new(bimap!(5 => 1, 1 => 5, 2 => 4, 4 => 2), ground.clone());
        let func = reverse_cycle.get_function();
        generator.add(func);
        // Call the iterator
        let mut gen_iter = generator.iter();
        debug_assert_eq!(gen_iter.next(), Some(vec![1, 2, 3, 4, 5]));
        debug_assert_eq!(gen_iter.next(), Some(vec![5, 4, 3, 2, 1]));
        debug_assert_eq!(gen_iter.next(), Some(vec![1, 2, 3, 4, 5]));
        debug_assert_eq!(gen_iter.next(), Some(vec![5, 4, 3, 2, 1]));
        debug_assert_eq!(gen_iter.next(), Some(vec![1, 2, 3, 4, 5]));
        debug_assert_eq!(gen_iter.next(), Some(vec![5, 4, 3, 2, 1]));
    }

    #[test]
    fn trace_aaaa() {
        let ground = vec![1, 2, 3, 4, 5];
        let mut generator = PeriodicGen::new();
        generator.set_start(&ground);
        let reverse_cycle = Cycle::new(bimap!(1 => 1, 2 => 2, 3 => 3, 4 => 4, 5 => 5), ground.clone());
        let func = reverse_cycle.get_function();
        generator.add(func);
        let mut gen_iter = generator.iter();
        debug_assert_eq!(gen_iter.next(), Some(vec![1, 2, 3, 4, 5]));
        debug_assert_eq!(gen_iter.next(), Some(vec![1, 2, 3, 4, 5]));
        debug_assert_eq!(gen_iter.next(), Some(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn trace_specific() {

        let ground = vec!["x_1", "x_2", "x_3", "x_4", "x_5"];
        let mut generator = PeriodicGen::new();
        generator.set_start(&vec!["x_2"]);
        let cycle = Cycle::new(bimap!("x_2" => "x_3", "x_3" => "x_4", "x_4" => "x_2"), ground.clone());
        let func = cycle.get_function();
        generator.add(func);

        let mut gen_iter = generator.iter();
        debug_assert_eq!(gen_iter.next(), Some(vec!["x_2"])); // "x_1", "x_2", "x_3", "x_4", "x_5"
        debug_assert_eq!(gen_iter.next(), Some(vec!["x_3"])); // "x_1", "x_3", "x_4", "x_2", "x_5"
        debug_assert_eq!(gen_iter.next(), Some(vec!["x_4"])); // "x_1", "x_4", "x_2", "x_3", "x_5"
        debug_assert_eq!(gen_iter.next(), Some(vec!["x_2"])); // "x_1", "x_2", "x_3", "x_4", "x_5"
        debug_assert_eq!(gen_iter.next(), Some(vec!["x_3"]));
    }
    #[test]
    fn trace_iter_collect() {
        let ground = vec![1, 2, 3, 4, 5];
        let mut generator = PeriodicGen::new();
        generator.set_start(&ground);
        let reverse_cycle = Cycle::new(bimap!(1 => 2, 2 => 3, 3 => 4, 4 => 5, 5 => 1), ground.clone());
        let func = reverse_cycle.get_function();
        generator.add(func);
        debug_assert_eq!(generator.simulate(0), vec![1, 2, 3, 4, 5]);
        debug_assert_eq!(generator.simulate(1), vec![1, 2, 3, 4, 5, 2, 3, 4, 5, 1]);
        debug_assert_eq!(generator.simulate(2), vec![1, 2, 3, 4, 5, 2, 3, 4, 5, 1, 3, 4, 5, 1, 2]);
        debug_assert_eq!(generator.simulate(3), vec![1, 2, 3, 4, 5, 2, 3, 4, 5, 1, 3, 4, 5, 1, 2, 4, 5, 1, 2, 3]);
        debug_assert_eq!(generator.simulate(4), vec![1, 2, 3, 4, 5, 2, 3, 4, 5, 1, 3, 4, 5, 1, 2, 4, 5, 1, 2, 3, 5, 1, 2, 3, 4]);
        debug_assert_eq!(generator.simulate(5), vec![1, 2, 3, 4, 5, 2, 3, 4, 5, 1, 3, 4, 5, 1, 2, 4, 5, 1, 2, 3, 5, 1, 2, 3, 4, 1, 2, 3, 4, 5]);
    }

    
}