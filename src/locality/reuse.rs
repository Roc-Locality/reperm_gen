use crate::math::combinations::{factorial, combinations};
use std::fmt::Debug;
use std::hash::Hash;
use std::collections::{HashMap, HashSet};

use petgraph::data;

/// This will calculate the forward distance.
/// This is done by looking ahead and refreshing the last seen index in its calculation.
/// (similar to live analysis, where we read backwards)
fn calculate_reuse_distance<T>(trace: &Vec<T>) -> Vec<i32>
where T: Clone+Eq+Hash+Debug
{
    let mut reuse_sets: HashMap<&T, HashSet<&T>> = HashMap::new();
    let mut reuse_distance: Vec<i32> = Vec::new();

    for access in trace.into_iter().rev() {
        reuse_sets.iter_mut().for_each(|(_, reuse_set)| {
            reuse_set.insert(access);
        });
        match reuse_sets.get_mut(access) {
            Some(reuse_set) => {
                let size = reuse_set.len() as i32;
                reuse_distance.push(size);
                reuse_set.clear();
            },
            None => {
                reuse_sets.insert(access, HashSet::new());
                reuse_distance.push(-1);
            },
        }
    }
    reuse_distance.reverse();
    reuse_distance
}

fn calculate_lru_hits<T>(trace: &Vec<T>, cache_size: usize) -> usize
    where T: Clone+Eq+Hash+Debug
{
    calculate_reuse_distance(trace).into_iter()
        .filter(|x| *x != -1)
        .filter(|x| *x <= cache_size as i32)
        .count()
}

fn calculate_lru_hits_formula(data_items: i128, cache_size: i128, hits: i128) -> i128 {
    if 2 * cache_size <=  data_items + hits {
        let m = data_items as i128;
        let c = cache_size as i128;
        let h = hits as i128;

        factorial(c) * combinations(c, h) * combinations(m - c, c - h) * factorial(m - c)
    } else {
        0
    }
}

fn calculate_dmc<T>(trace: &Vec<T>) -> f64
where T: Clone+Eq+Hash+Debug
{
    calculate_reuse_distance(trace).into_iter()
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
        debug_assert_eq!(rd, vec![4, 2, 3, -1, -1, -1, -1]);
    }

    #[test]
    fn simple_trace_hits() {
        let trace = vec!["a", "b", "c", "b", "d", "c", "a"];
        let cache_size = 3;
        let num_hits = calculate_lru_hits(&trace, cache_size);
        debug_assert_eq!(num_hits, 2);
    }

    #[test]
    fn equivalent_comb_formula_1() {
        let data_items = 7;
        let cache_size = 5;

        let a: i128 = (0..=cache_size).into_iter()
            .map(|h| calculate_lru_hits_formula(data_items, cache_size, h))
            .sum();
        
        debug_assert_eq!(a, factorial(data_items));
    }

    #[test]
    fn equivalent_comb_formula_2() {
        let data_items = 19;
        let cache_size = 2;

        let a: i128 = (0..=cache_size).into_iter()
            .map(|h| calculate_lru_hits_formula(data_items, cache_size, h))
            .sum();
        
        debug_assert_eq!(a, factorial(data_items));
    }

    #[test]
    fn equivalent_comb_formula_3() {
        let data_items = 33;
        let cache_size = 4;

        let a: i128 = (0..=cache_size).into_iter()
            .map(|h| calculate_lru_hits_formula(data_items, cache_size, h))
            .sum();
        
        debug_assert_eq!(a, factorial(data_items));
    }
    

    #[test]
    fn cyclic() {
        let trace = vec!["a", "b", "c", "d", "a", "b", "c", "d"];
        let rd = calculate_reuse_distance(&trace);
        debug_assert_eq!(rd, vec![4, 4, 4, 4, -1, -1, -1, -1]);
    }

    #[test]
    fn sawtooth() {
        let trace = vec!["a", "b", "c", "d", "d", "c", "b", "a"];
        let rd = calculate_reuse_distance(&trace);
        debug_assert_eq!(rd, vec![4, 3, 2, 1, -1, -1, -1, -1]);
    }

    #[test]
    fn simple_trace_dmc() {
        let trace = vec!["a", "b", "c", "b", "d", "c", "a"];
        let dmc = calculate_dmc(&trace);
        
        debug_assert_eq!(dmc, f64::sqrt(4.0) + f64::sqrt(3.0) + f64::sqrt(2.0));
    }
}