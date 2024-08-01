use std::collections::VecDeque;
use std::hash::Hash;
use std::fmt::Debug;
use std::cmp::min;

use crate::group::cycle::Cycle;
use crate::group::group::Group;
use crate::group::symmetric::SymmetricGroup;


fn chain_find<V, F, O>(group: &SymmetricGroup<V>, start: Cycle<V>, locality_calc: F, maxlen: usize) -> Vec<Cycle<V>>
where 
    V: Clone+Copy+Hash+Eq+PartialEq+Debug+PartialOrd,
    F: Fn(&Cycle<V>) -> O,
    O: PartialOrd+PartialEq+Ord
{
    let generators = group.get_generator();
    let mut res = VecDeque::new();
    res.push_back(start.clone());
    let mut curr_length: usize = start.inversions();
    let max_length = min((group.ground_size() * (group.ground_size() + 1)) / 2, maxlen);
    while curr_length < max_length {
        let node = res.back().unwrap();
        let left_map: Vec<Cycle<V>> = generators.iter().map(|gen| {node.clone() * gen.clone()}).collect();
        let right_map: Vec<Cycle<V>> = generators.iter().map(|gen| {gen.clone() * node.clone()}).collect();
        let total = [&left_map[..], &right_map[..]].concat();
        let max_locality = total.iter()
            .filter(|x| locality_calc(x) > locality_calc(node))
            .max_by(|a, b| {
                locality_calc(a).cmp(&locality_calc(b))
            });
        if let Some(u) = max_locality {
            res.push_back(u.clone());
        } else {
            break;
        }
        curr_length += 1
    }

    return res.into_iter().collect()
}



#[cfg(test)]
mod tests {
    use std::usize;
    
    use crate::generator::generator::Generator;
    use crate::generator::periodic::PeriodicGen;
    use crate::group::cycle::Cycle;
    use crate::group::group::Group;
    use crate::group::symmetric::SymmetricGroup; 
    use crate::locality::chainfind::chain_find;
    use crate::locality::reuse::calculate_lru_hits;

    #[test]
    fn s4_chain() {
        let ground = vec![1, 2, 3, 4];
        let s_m = SymmetricGroup::new(ground.len(), ground.clone());
        
        let hits_ranking = |cycle: &Cycle<_>| {
            let mut generator = PeriodicGen::new();
            generator.set_start(&ground.clone());
            generator.add(cycle.get_function());
            (calculate_lru_hits(&generator.simulate(1), 3), calculate_lru_hits(&generator.simulate(1), 2), calculate_lru_hits(&generator.simulate(1), 1))  
        };
        let identity = s_m.identity();
        let chain: Vec<Cycle<i32>> = chain_find(&s_m, identity, |retraversal| {hits_ranking(retraversal)}, usize::MAX);
        debug_assert_eq!(&chain, &[
            s_m.create_vec(vec![vec![]]),
            s_m.create_vec(vec![vec![3, 4]]),
            s_m.create_vec(vec![vec![2, 3, 4]]),
            s_m.create_vec(vec![vec![1, 2, 3, 4]]),
            s_m.create_vec(vec![vec![1, 2, 4]]),
            s_m.create_vec(vec![vec![1, 3, 2, 4]]),
            s_m.create_vec(vec![vec![1, 4], vec![2, 3]]),
        ]);
    }
}

