use std::fmt::Debug;
use std::hash::Hash;
use std::collections::{HashMap, HashSet};

/// This will calculate the forward distance.
/// This is done by looking ahead and refreshing the last seen index in its calculation.
/// (similar to live analysis, where we read backwards)
fn calculate_reuse_distance<T>(trace: &Vec<T>) -> Vec<i32>
where T: Clone+Eq+Hash+Debug
{
    let mut reuse_sets: HashMap<T, HashSet<T>> = HashMap::new();
    let mut reuse_distance: Vec<i32> = Vec::new();

    for access in trace.into_iter().rev() {
        reuse_sets.iter_mut().for_each(|(_, reuse_set)| {
            reuse_set.insert(access.clone());
        });
        match reuse_sets.get_mut(access) {
            Some(reuse_set) => {
                let size = reuse_set.len() as i32;
                reuse_distance.push(size);
                reuse_set.clear();
            },
            None => {
                reuse_sets.insert(access.clone(), HashSet::new());
                reuse_distance.push(-1);
            },
        }
    }
    reuse_distance.reverse();
    reuse_distance
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
    use crate::reuse::calculate_dmc;
    use crate::reuse::calculate_reuse_distance;
    
    #[test]
    fn simple_trace() {
        let trace = vec!["a", "b", "c", "b", "d", "c", "a"];
        let rd = calculate_reuse_distance(&trace);
        debug_assert_eq!(rd, vec![4, 2, 3, -1, -1, -1, -1]);
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