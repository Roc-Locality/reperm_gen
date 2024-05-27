///A generator is an interface that tells us that given a base set
///It will transform the set into another set as long as the operation is closed.
///Meaning that the set yielded is from same domain as the single operand.
pub trait Generator<T> 
where 
    T: PartialEq+Sized+Clone
{
    fn start(&self) -> Vec<T>;
    fn set_start(&self, start: &Vec<T>);
    /// This is supposed to manage adding some function to the generator.
    fn add(&self, f: fn(T) -> T);
    /// This clears the list of permutations.
    fn clear(&self);
    fn iter(&self) -> GenIter<T>;
}

pub struct GenIter<T> {

}