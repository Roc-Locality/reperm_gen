use crate::math::combinations::{combinations, factorial};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

/// This will calculate the forward distance.
/// This is done by looking ahead and refreshing the last seen index in its calculation.
/// (similar to live analysis, where we read backwards)
#[allow(unused)]
fn calculate_reuse_distance<T>(trace: &[T]) -> Vec<i32>
where
    T: Clone + Eq + Hash + Debug,
{
    let n = trace.len() / 2 + 1;
    let mut reuse_sets: HashMap<&T, HashSet<&T>> = HashMap::with_capacity(n);
    let mut reuse_distance: Vec<i32> = Vec::new();

    for access in trace.iter() {
        reuse_sets.iter_mut().for_each(|(_, reuse_set)| {
            reuse_set.insert(access);
        });
        match reuse_sets.get_mut(access) {
            Some(reuse_set) => {
                let size = reuse_set.len() as i32;
                reuse_distance.push(size);
                reuse_set.clear();
            }
            None => {
                reuse_sets.insert(access, HashSet::with_capacity(n));
                reuse_distance.push(-1);
            }
        }
    }

    reuse_distance
}

pub fn calculate_lru_hits<T>(trace: &[T], cache_size: usize) -> usize
where
    T: Clone + Eq + Hash + Debug,
{
    calculate_reuse_distance(trace)
        .into_iter()
        .filter(|x| {
            let y = *x;
            y != -1 && y <= cache_size as i32
        })
        .count()
}

#[allow(unused)]
fn calculate_lru_hits_formula(data_items: i128, cache_size: i128, hits: i128) -> i128 {
    if 2 * cache_size <= data_items + hits {
        let m = data_items;
        let c = cache_size;
        let h = hits;

        factorial(c) * combinations(c, h) * combinations(m - c, c - h) * factorial(m - c)
    } else {
        0
    }
}

#[allow(unused)]
fn calculate_dmc<T>(trace: &[T]) -> f64
where
    T: Clone + Eq + Hash + Debug,
{
    calculate_reuse_distance(trace)
        .into_iter()
        .filter(|x| *x != -1)
        .map(|x| (x as f64).sqrt())
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::locality::reuse::calculate_dmc;
    use crate::locality::reuse::calculate_lru_hits;
    use crate::locality::reuse::calculate_reuse_distance;
    use crate::locality::reuse::factorial;

    use super::calculate_lru_hits_formula;

    #[test]
    fn simple_trace() {
        let trace = vec!["a", "b", "c", "b", "d", "c", "a"];
        let rd = calculate_reuse_distance(&trace);
        debug_assert_eq!(rd, vec![-1, -1, -1, 2, -1, 3, 4,]);
    }

    #[test]
    fn simple_trace_hits() {
        let trace = vec!["a", "b", "c", "b", "d", "c", "a"];
        let cache_size = 3;
        let num_hits = calculate_lru_hits(&trace, cache_size);
        debug_assert_eq!(num_hits, 2);
    }

    #[test]
    fn simple_trace_hits_1() {
        let cache_size = 2;
        debug_assert_eq!(
            calculate_lru_hits(&vec![1, 2, 3, 4, 1, 2, 3, 4], cache_size),
            0
        );
        debug_assert_eq!(
            calculate_lru_hits(&vec![1, 2, 3, 4, 2, 1, 3, 4], cache_size),
            0
        );
        debug_assert_eq!(
            calculate_lru_hits(&vec![1, 2, 3, 4, 1, 3, 2, 4], cache_size),
            0
        );
        debug_assert_eq!(
            calculate_lru_hits(&vec![1, 2, 3, 4, 1, 2, 4, 3], cache_size),
            0
        );
    }

    #[test]
    fn equivalent_comb_formula_1() {
        let data_items = 7;
        let cache_size = 5;

        let a: i128 = (0..=cache_size)
            .into_iter()
            .map(|h| calculate_lru_hits_formula(data_items, cache_size, h))
            .sum();

        debug_assert_eq!(a, factorial(data_items));
    }

    #[test]
    fn equivalent_comb_formula_2() {
        let data_items = 19;
        let cache_size = 2;

        let a: i128 = (0..=cache_size)
            .into_iter()
            .map(|h| calculate_lru_hits_formula(data_items, cache_size, h))
            .sum();

        debug_assert_eq!(a, factorial(data_items));
    }

    #[test]
    fn equivalent_comb_formula_3() {
        let data_items = 33;
        let cache_size = 4;

        let a: i128 = (0..=cache_size)
            .into_iter()
            .map(|h| calculate_lru_hits_formula(data_items, cache_size, h))
            .sum();

        debug_assert_eq!(a, factorial(data_items));
    }

    #[test]
    fn cyclic() {
        let trace = vec!["a", "b", "c", "d", "a", "b", "c", "d"];
        let rd = calculate_reuse_distance(&trace);
        debug_assert_eq!(rd, vec![-1, -1, -1, -1, 4, 4, 4, 4]);
    }

    #[test]
    fn sawtooth() {
        let trace = vec!["a", "b", "c", "d", "d", "c", "b", "a"];
        let rd = calculate_reuse_distance(&trace);
        debug_assert_eq!(rd, vec![-1, -1, -1, -1, 1, 2, 3, 4]);
    }

    #[test]
    fn simple_trace_dmc() {
        let trace = vec!["a", "b", "c", "b", "d", "c", "a"];
        let dmc = calculate_dmc(&trace);

        debug_assert_eq!(dmc, f64::sqrt(4.0) + f64::sqrt(3.0) + f64::sqrt(2.0));
    }
}
