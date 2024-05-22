use std::collections::HashSet;

/// A Group is a set system with some set G, the ground set and an arbitrary binary operator +.
/// In particular it follows three properties:
/// Associativity
/// Identity
/// Inverse
pub trait Group<T: PartialEq+Sized+Copy> {
    fn get_set(&self) -> HashSet<T>;
    fn op(&self, a: T, b: T) -> T;

    fn identity(&self) -> T;
    fn inverse(&self, e: T) -> T;

    fn order(&self) -> usize;
    fn get_generator(&self) -> Vec<T>;
    fn iter(&self, start: T) -> GroupIter<T> where Self: Sized, T: Copy {
        GroupIter::new(start, self)
    }
}

struct GroupIter<'a, T> {
    start: T,
    curr: T,
    group: &'a dyn Group<T>,
}

impl<'a, T> GroupIter<'a, T>
where 
    T: Copy 
{
    fn new(curr: T, group: &'a dyn Group<T>) -> GroupIter<'a, T> {
        GroupIter {
            start: curr,
            curr: curr,
            group: group,
        }
    }
}

impl<'a, T> Iterator for GroupIter<'a, T> 
where 
    T: PartialEq+Copy
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.curr = self.group.op(self.curr, self.curr);
        if self.curr != self.start {
            Some(self.curr)
        } else {
            None
        }
    }
}