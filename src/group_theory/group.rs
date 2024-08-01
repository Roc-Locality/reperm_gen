use std::collections::{HashSet, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Mul;

/// A Group is a set system with some set G, the ground set and an arbitrary binary operator +.
/// In particular it follows three properties:
/// Associativity
/// Identity
/// Inverse
pub trait Group<T: Hash + Eq + Sized + Clone> {
    /// This gives a true search throughout the entire group set (hopefully finite).
    /// This is done by using a BFS, starting at the identity of the element, and keep doing operations until you can't anymore.
    /// This means that for cyclic groups, this is the same as iter(get_generator()[0]), where there is only 1 generator.
    fn get_set(&self) -> HashSet<T>
    where
        T: Debug,
    {
        let generators: Vec<T> = self.get_generator();
        let mut elements = HashSet::new();
        //construct a bfs, with the identity element intiially in it.
        let mut q = VecDeque::from([self.identity()]);
        while !q.is_empty() {
            if let Some(element) = q.pop_front() {
                if elements.contains(&element) {
                    continue;
                }
                elements.insert(element.clone());
                for gen in generators.iter() {
                    let new_element = self.op(gen.clone(), element.clone());
                    q.push_front(new_element);
                }
            }
        }
        elements
    }
    fn op(&self, a: T, b: T) -> T;

    fn identity(&self) -> T;
    fn inverse(&self, e: T) -> T;

    fn order(&self) -> i32;
    fn get_generator(&self) -> Vec<T>;

    /// This iterates one by one throughout the group set. For groups that aren't cyclic, this is not guaranteed to visit every single element.
    fn iter(&self, start: T) -> GroupIter<T>
    where
        Self: Sized,
        T: Clone,
    {
        GroupIter::new(start, self)
    }
}

pub struct GroupIter<'a, T> {
    start: T,
    curr: T,
    group: &'a dyn Group<T>,
}

impl<'a, T> GroupIter<'a, T>
where
    T: Clone,
{
    fn new(curr: T, g: &'a dyn Group<T>) -> GroupIter<'a, T> {
        GroupIter {
            start: curr.clone(),
            curr: curr.clone(),
            group: g,
        }
    }
}

impl<'a, T> Iterator for GroupIter<'a, T>
where
    T: Eq + Hash + PartialEq + Clone + Mul<&'a T> + Mul<T>,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.curr = self.group.op(self.curr.clone(), self.start.clone());
        if self.curr != self.start {
            Some(self.curr.clone())
        } else {
            None
        }
    }
}
