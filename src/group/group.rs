use std::ops::Mul;
use std::collections::HashSet;

/// A Group is a set system with some set G, the ground set and an arbitrary binary operator +.
/// In particular it follows three properties:
/// Associativity
/// Identity
/// Inverse
pub trait Group<T: PartialEq+Sized+Clone> {
    fn get_set(&self) -> HashSet<T>;
    fn op(&self, a: T, b: T) -> T;

    fn identity(&self) -> T;
    fn inverse(&self, e: T) -> T;

    fn order(&self) -> i32;
    fn get_generator(&self) -> Vec<T>;
    fn iter(&self, start: T) -> GroupIter<T> 
    where 
        Self: Sized, T: Clone 
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
    T: Clone 
{
    fn new(curr: T, group: &'a dyn Group<T>) -> GroupIter<'a, T> {
        GroupIter {
            start: curr.clone(),
            curr: curr.clone(),
            group: group,
        }
    }
}

impl<'a, T> Iterator for GroupIter<'a, T> 
where 
    T: PartialEq+Clone+Mul<&'a T>+Mul<T>
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