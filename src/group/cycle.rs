use std::ops::Mul;

///For permutations
pub struct Cycle<T> {
    /// data = [1, 2, 3,], which is a cycle of order 3
    data: Vec<T>,
    ///this is the max cycle size
    n: usize,
}

impl<T> Cycle<T> 
where
    T: Clone
{
    fn new(data: Vec<T>) -> Self {
        Cycle {
            data: data,
            n: data.len(),
        }
    }

    fn inverse(&self) -> Self {
        let co = self.data.to_vec();
        co.reverse();
        Cycle {
            data: co,
            n: self.n,
        }
    }
}

impl<T> Mul for Cycle<T> {
    type Output = Cycle<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
impl<T> PartialEq for Cycle<T> {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}