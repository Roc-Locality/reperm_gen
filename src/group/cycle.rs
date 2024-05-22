use std::ops::Mul;

///For permutations
#[derive(Clone, Copy)]
pub struct Cycle<T: Copy> {
    /// data = [1, 2, 3,], which is a cycle of order 3
    data: Vec<T>,
    ///this is the max cycle size
    n: usize,
}

impl<T: Copy> Cycle<T> {
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

impl<T: Copy> Mul for Cycle<T> {
    type Output = Cycle<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<T: Copy> PartialEq for Cycle<T> {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}