use crate::bimap;

use bimap::BiMap;
use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};
use std::collections::HashSet;
use std::fmt::{self, Debug};
use std::hash::Hash;
use std::ops::Mul;

///For permutations
#[derive(Clone, Debug, Eq)]
pub struct Cycle<T>
where
    T: Clone + Hash + Eq + 'static,
{
    ground: Vec<T>,
    ///this is the max cycle size
    n: usize,
    /// The main way to define a bimap
    map: BiMap<T, T>,
    //For hash
    h: Vec<T>,
}

impl<T> Cycle<T>
where
    T: Debug + Clone + Eq + Hash + 'static,
{
    pub fn new(map: BiMap<T, T>, ground: Vec<T>) -> Self {
        let mut new_map = map.clone();
        let size = ground.len();
        if map.len() != size {
            ground
                .iter()
                .filter(|g| !map.contains_left(g))
                .for_each(|g: &T| {
                    new_map.insert(g.clone(), g.clone());
                });
        }
        Cycle {
            ground: ground.clone(),
            n: size,
            map: new_map.clone(),
            h: ground
                .into_iter()
                .map(|x| new_map.get_by_left(&x).unwrap())
                .cloned()
                .collect(),
        }
    }

    pub fn from(vec: Vec<Vec<T>>, ground: Vec<T>) -> Self {
        let mut set: HashSet<T> = HashSet::from_iter(ground.iter().cloned());
        let mut map = BiMap::new();
        for cycle in vec.into_iter() {
            for window in cycle.windows(2) {
                let prev = window[0].clone();
                let curr = window[1].clone();

                set.remove(&prev);
                map.insert(prev, curr);
            }
            if let (Some(last), Some(first)) = (cycle.last(), cycle.first()) {
                set.remove(last);
                map.insert(last.clone(), first.clone());
            }

            // Handle unchanged elements:
            for e in set.iter() {
                map.insert(e.clone(), e.clone());
            }
        }

        Cycle {
            ground: ground.clone(),
            n: map.len(),
            map: map.clone(),
            h: ground
                .into_iter()
                .map(|x| map.get_by_left(&x).unwrap())
                .cloned()
                .collect(),
        }
    }

    pub fn from_retraversal(vec: &[T], ground: &Vec<T>) -> Self {
        // Commenting out checks for big numbers
        // assert_eq!(vec.len(), ground.len());
        // assert_eq!(
        //     vec.iter().collect::<HashSet<_>>(),
        //     ground.iter().collect::<HashSet<_>>()
        // );

        Self::new(
            vec.iter().zip(ground).fold(BiMap::new(), |mut map, ab| {
                map.insert(ab.1.clone(), ab.0.clone());
                map
            }),
            ground.clone(),
        )
    }

    pub fn get_ground(&self) -> Vec<T> {
        self.ground.clone()
    }

    pub fn inverse(&self) -> Self {
        let mut co = BiMap::new();
        for g in self.ground.clone() {
            co.insert(g.clone(), self.map.get_by_left(&g).unwrap().clone());
        }
        Cycle::new(co, self.ground.clone())
    }

    pub fn eval(&self, i: T) -> T {
        match self.map.get_by_left(&i) {
            Some(res) => res.clone(),
            _ => i, // if it doesn't match, we will make an assumption here that it will just return the same thing
        }
    }

    pub fn get_function(&self) -> Box<dyn Fn(T) -> T> {
        let m = self.map.clone();
        Box::new(move |e| m.get_by_left(&e).unwrap().clone())
    }

    pub fn get_retraversal_str(&self) -> String
    where
        T: ToString,
    {
        self.ground
            .iter()
            .map(|y| self.eval(y.clone()))
            .map(|y| y.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    /// This yields the cycle representation, aka (123),(435), etc
    /// If you pass the output to a #from method, you should get the same object.
    /// If show_one is True, then we should also show the full cycle representation (aka, include the 1-cycles)
    /// If show_ene is False, show the normal representation.
    pub fn get_cycle_representation(&self, show_one: bool) -> Vec<Vec<T>> {
        let mut visited = HashSet::new();
        let mut cycles = Vec::new();
        for g in self.ground.clone() {
            let mut cycle = Vec::new();
            let mut curr: &T = &g;
            while !visited.contains(curr) {
                visited.insert(curr.clone());
                cycle.push(curr.clone());
                curr = self.map.get_by_left(curr).unwrap();
            }

            if !cycle.is_empty() && (cycle.len() != 1 || show_one) {
                cycles.push(cycle);
            }
        }
        cycles
    }

    pub fn inversions(&self) -> usize
    where
        T: PartialOrd,
    {
        let mut inv = 0;
        for i in (0..self.n).rev() {
            for j in i + 1..self.n {
                if self.h[i] > self.h[j] {
                    inv += 1;
                }
            }
        }
        inv
    }

    pub fn display(&self) -> String
    where
        T: ToString,
    {
        self.get_cycle_representation(false)
            .iter()
            .map(|cycle| {
                format!(
                    "({})",
                    cycle
                        .iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<String>>()
                        .join(",")
                )
            })
            .collect::<Vec<String>>()
            .join("")
    }
}

impl<T> Serialize for Cycle<T>
where
    T: Clone + Hash + Eq + ToString + Debug,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Start a struct serialization
        let mut state = serializer.serialize_map(Some(2))?;
        state.serialize_entry("n", &self.n)?;
        state.serialize_entry("value", &self.get_retraversal_str())?;
        state.end()
    }
}

impl<T> Mul for Cycle<T>
where
    T: Clone + Hash + Eq + Debug,
{
    type Output = Cycle<T>;

    fn mul(self, rhs: Cycle<T>) -> Self::Output {
        if self.ground.len() != rhs.ground.len() {
            panic!(
                "The cycles being multiplied have the wrong ground sets! {:?} vs {:?}",
                self.ground.len(),
                rhs.ground.len()
            );
        }
        let same_ground = self.ground.clone();

        let mut new_map: BiMap<T, T> = BiMap::new();

        for g in self.ground {
            let pcyc = match rhs
                .map
                .get_by_left(&g)
                .and_then(|f| self.map.get_by_left(f))
            {
                Some(res) => res.clone(),
                None => panic!(
                    "Compositions should not be empty! query: {:?} rhs: {:?}, lhs: {:?}",
                    g, rhs.map, self.map
                ),
            };
            new_map.insert(g, pcyc);
        }

        Cycle::new(new_map, same_ground)
    }
}

impl<T> PartialEq for Cycle<T>
where
    T: Clone + Hash + Eq,
{
    fn eq(&self, other: &Self) -> bool {
        self.map.eq(&other.map)
    }
}

impl<T> Hash for Cycle<T>
where
    T: Clone + Hash + Eq + Debug,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        //self.ground.hash(state);
        self.n.hash(state);
        self.h.hash(state);
    }
}
impl<T> fmt::Display for Cycle<T>
where
    T: Clone + Hash + Eq + Debug + ToString,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.display())
    }
}

/// Tests, mainly associative

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::group_theory::{cycle::Cycle, group::Group, symmetric::SymmetricGroup};

    #[test]
    fn construction1() {
        let ground = vec![1, 2, 3, 4, 5];
        let f = Cycle::from(vec![vec![1, 3], vec![4, 5]], ground.clone());
        let g = Cycle::from(vec![vec![1, 2, 5], vec![3, 4]], ground.clone());
        // f = 1 3 4 5
        //     3 1 5 4
        // g = 1 2 5 3 4
        //     2 5 1 4 3
        debug_assert_eq!(f.map, crate::bimap![1 => 3, 3 => 1, 4 => 5, 5 => 4, 2 => 2]);
        debug_assert_eq!(g.map, crate::bimap![1 => 2, 2 => 5, 5 => 1, 3 => 4, 4 => 3]);
    }

    #[test]
    fn construction2() {
        let ground = vec![1, 2, 3, 4, 5];
        let f = Cycle::from(vec![vec![1, 2]], ground.clone());
        // f = 1 3 4 5
        //     3 1 5 4
        debug_assert_eq!(f.map, crate::bimap![1 => 2, 2 => 1, 3 => 3, 4 => 4, 5 => 5]);
    }

    #[test]
    fn construction3() {
        let ground = vec![1, 2, 3, 4, 5];
        let f = Cycle::from(vec![vec![1, 2], vec![3, 4]], ground.clone());

        debug_assert_eq!(
            f.get_cycle_representation(true),
            vec![vec![1, 2], vec![3, 4], vec![5]]
        );
        debug_assert_eq!(
            f.get_cycle_representation(false),
            vec![vec![1, 2], vec![3, 4]]
        );
    }

    #[test]
    fn construction4() {
        let ground = vec![1, 2, 3, 4, 5];
        let f = Cycle::from(vec![vec![1, 5], vec![2, 4], vec![3]], ground.clone());

        debug_assert_eq!(
            f.get_cycle_representation(true),
            vec![vec![1, 5], vec![2, 4], vec![3]]
        );
        debug_assert_eq!(
            f.get_cycle_representation(false),
            vec![vec![1, 5], vec![2, 4]]
        );
    }

    #[test]
    fn construction5() {
        let ground = vec![1, 2, 3, 4, 5];
        let f = Cycle::from(vec![vec![1, 5], vec![2, 4], vec![3]], ground.clone());
        let serialized = serde_json::to_string_pretty(&f).unwrap();
        debug_assert_eq!(serialized, "{\n  \"n\": 5,\n  \"value\": \"5,4,3,2,1\"\n}");
    }

    #[test]
    fn construction6() {
        let ground = vec![1, 2, 3, 4, 5];
        let f = Cycle::from(vec![vec![]], ground.clone());
        debug_assert_eq!(&f.get_retraversal_str(), "1,2,3,4,5");
    }

    #[test]
    fn display1() {
        let ground = vec![1, 2, 3, 4, 5];
        let f = Cycle::from(vec![vec![1, 5], vec![2, 4], vec![3]], ground.clone());
        debug_assert_eq!(f.display(), "(1,5)(2,4)");
    }

    #[test]
    fn fn_construction1() {
        let ground = vec![1, 2, 3, 4, 5];
        let f = Cycle::from(vec![vec![1, 5], vec![4, 2]], ground.clone());
        let func = f.get_function();

        debug_assert_eq!(func(3), 3);
        debug_assert_eq!(func(1), 5);
        debug_assert_eq!(func(5), 1);
        debug_assert_eq!(func(2), 4);
        debug_assert_eq!(func(4), 2);
    }

    #[test]
    fn fn_construction2() {
        let ground = vec![1, 2, 3, 4, 5];
        let f = Cycle::from(vec![vec![]], ground.clone());
        let func = f.get_function();

        debug_assert_eq!(func(1), 1);
        debug_assert_eq!(func(2), 2);
        debug_assert_eq!(func(3), 3);
        debug_assert_eq!(func(4), 4);
        debug_assert_eq!(func(5), 5);
    }

    #[test]
    fn eq1() {
        let ground = vec![1, 2, 3, 4, 5];
        let f = Cycle::from(vec![vec![1, 2, 3, 4, 5]], ground.clone());
        let g = Cycle::from(vec![vec![1, 2, 3, 4, 5]], ground.clone());
        debug_assert_eq!(f, g);
    }

    #[test]
    fn eq2() {
        let ground = vec![1, 2, 3, 4, 5];
        let f = Cycle::from(vec![vec![1, 2]], ground.clone());
        let g = Cycle::from(vec![vec![2, 1]], ground.clone());
        debug_assert_eq!(f, g);
    }

    #[test]
    fn contains_hash() {
        let ground = vec![1, 2, 3, 4, 5];
        let f = Cycle::from(vec![vec![1, 2, 3, 4, 5]], ground.clone());

        let g = Cycle::from(vec![vec![1, 2, 3, 4, 5]], ground.clone());
        debug_assert_eq!(f.clone(), g.clone());
        let mut set = HashSet::new();
        set.insert(f);
        debug_assert_eq!(set.contains(&g), true);
    }

    #[test]
    fn mul_eq1() {
        let ground = vec![1, 2, 3, 4, 5];
        let f = Cycle::from(vec![vec![1, 3], vec![4, 5]], ground.clone());
        let g = Cycle::from(vec![vec![1, 2, 5], vec![3, 4]], ground.clone());
        let fg: Cycle<_> = Cycle::from(vec![vec![1, 2, 4], vec![3, 5]], ground.clone());

        debug_assert_eq!((f * g).map, fg.map);
    }

    #[test]
    fn apply1() {
        let ground = vec![1, 2, 3, 4, 5];
        let f = Cycle::from(vec![vec![1, 5], vec![2, 4]], ground.clone());
        let output: Vec<i32> = ground.into_iter().map(|x| f.eval(x)).collect();
        debug_assert_eq!(output, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn inversions() {
        let ground = vec![1, 2, 3, 4];
        let s_4 = SymmetricGroup::new(ground.len(), ground);
        debug_assert_eq!(s_4.identity().inversions(), 0);
        let gen0 = s_4.create_vec(vec![vec![1, 2]]);
        let gen1 = s_4.create_vec(vec![vec![2, 3]]);
        let gen2 = s_4.create_vec(vec![vec![3, 4]]);
        debug_assert_eq!((s_4.identity() * gen0.clone()).inversions(), 1);
        debug_assert_eq!((s_4.identity() * gen1.clone()).inversions(), 1);
        debug_assert_eq!((s_4.identity() * gen2.clone()).inversions(), 1);
        debug_assert_eq!((gen0.clone() * s_4.identity()).inversions(), 1);
        debug_assert_eq!((gen1.clone() * s_4.identity()).inversions(), 1);
        debug_assert_eq!((gen2.clone() * s_4.identity()).inversions(), 1);

        debug_assert_eq!(
            (s_4.identity() * gen0.clone() * gen1.clone()).inversions(),
            2
        );
        debug_assert_eq!(
            (s_4.identity() * gen1.clone() * gen0.clone()).inversions(),
            2
        );
        debug_assert_eq!(
            (s_4.identity() * gen1.clone() * gen2.clone()).inversions(),
            2
        );
        debug_assert_eq!(
            (s_4.identity() * gen1.clone() * gen2.clone()).inversions(),
            2
        );
        debug_assert_eq!(
            (s_4.identity() * gen2.clone() * gen1.clone()).inversions(),
            2
        );

        debug_assert_eq!(s_4.create_vec(vec![vec![1, 4], vec![2, 3]]).inversions(), 6);
    }
}
