///A generator is an interface that tells us that given a base set
///It will transform the set into another set as long as the operation is closed.
///Meaning that the set yielded is from same domain as the single operand.
///This generator is supposed to be as self contained as possible, so that if you want
///to add other ways of manipulating the trace outside of using a symmetric group.
pub trait Generator<'a, T> 
where 
    T: PartialEq+Sized+Clone
{
    fn start(&self) -> Vec<T>;
    fn set_start(&mut self, start: &Vec<T>);
    /// This is supposed to manage adding some function to the generator.
    fn add(&mut self, f: Box<dyn Fn(T) -> T>);
    /// This clears the list of permutations.
    fn clear(&mut self);
    fn iter(&'a self) -> Box<dyn Iterator<Item = Vec<T>> + 'a>;
    /// This runs iter m times, then returns the total vector of the result.
    fn simulate(&'a self, m: usize) -> Vec<T> {
        let mut iter = self.iter();
        let mut out = Vec::new();
        for _ in 0..=m {
            if let Some(mut next_trace) = iter.next() {
                out.append(&mut next_trace);
            }
        }
        out
    } 
}
