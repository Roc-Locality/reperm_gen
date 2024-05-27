///A generator is an interface that tells us that given a base set
///It will transform the set into another set as long as the operation is closed.
///Meaning that the set yielded is from same domain as the single operand.
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
}
