use std::ops::Mul;

///For permutations
#[derive(Clone)]
pub struct Cycle<T: Clone> {
    ///this is the max cycle size
    n: usize,
    /// data = [1, 2, 3,], which is a cycle of order 3
    data: Vec<T>,
    
}

impl<T: Clone> Cycle<T> {
    fn new(data: &Vec<T>) -> Self {
        Cycle { 
            data: data.to_vec(),
            n: data.len(),
        }
    }

    fn inverse(&self) -> Self {
        let mut co = self.data.to_vec();
        co.reverse();
        Cycle {
            data: co,
            n: self.n,
        }
    }
}

impl<T: Clone> Mul<T> for Cycle<T> {
    type Output = Cycle<T>;

    fn mul(self, rhs: T) -> Self::Output {
        todo!()
    }
}

impl<'a, 'b, T: Clone> Mul<&'b T> for &'a Cycle<T> {
    type Output = Cycle<T>;

    fn mul(self, rhs: &'b T) -> Self::Output {
        todo!()
    }
}

impl<T: Copy> PartialEq for Cycle<T> {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}