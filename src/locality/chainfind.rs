use std::cmp::min;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;

use serde::Serialize;

use crate::group_theory::cycle::Cycle;
use crate::group_theory::group::Group;
use crate::group_theory::symmetric::SymmetricGroup;

#[derive(Serialize, Debug)]
pub struct ChainFindResult<V>
where
    V: Clone + Copy + Hash + Eq + PartialEq + Debug + PartialOrd + ToString + 'static,
{
    pub length_non_unique: usize,
    pub length_chain: usize,
    pub chain: Vec<Cycle<V>>,
    pub non_unique_choices: HashMap<String, Vec<Cycle<V>>>,
}

#[allow(unused)]
pub fn chain_find<V, F, O>(
    group: &SymmetricGroup<V>,
    start: Cycle<V>,
    locality_calc: F,
    maxlen: usize,
) -> ChainFindResult<V>
where
    V: Clone + Copy + Hash + Eq + PartialEq + Debug + PartialOrd + ToString,
    F: Fn(&Cycle<V>) -> O,
    O: PartialOrd + PartialEq,
{
    let generators = group.get_generator();
    let mut res = VecDeque::new();
    res.push_back(start.clone());
    let mut curr_length: usize = start.inversions();
    let max_length = min(
        (group.ground_size() * (group.ground_size() + 1)) / 2,
        maxlen,
    );
    let mut non_unique_map = HashMap::with_capacity(20);
    let mut non_unique = 0;
    while curr_length < max_length {
        let node = res.back().unwrap();
        let left_map: Vec<Cycle<V>> = generators
            .iter()
            .map(|gen| node.clone() * gen.clone())
            .collect();
        let right_map: Vec<Cycle<V>> = generators
            .iter()
            .map(|gen| gen.clone() * node.clone())
            .collect();
        let total = [&left_map[..], &right_map[..]].concat();
        let mut max_locality = total
            .iter()
            .filter(|x| x.inversions() == node.inversions() + 1)
            .collect::<HashSet<_>>();
        let max_locality: Vec<_> = max_locality.iter().cloned().collect();
        if let Some(&first) = max_locality.first() {
            if max_locality.len() > 1
                && locality_calc(first) == locality_calc(max_locality.get(1).unwrap())
            {
                non_unique += 1;

                non_unique_map.entry(first.get_retraversal_str()).or_insert_with(Vec::default);
                let update = non_unique_map
                    .get_mut(&first.get_retraversal_str())
                    .unwrap();
                max_locality
                    .iter()
                    .filter(|x| locality_calc(x) == locality_calc(first))
                    .for_each(|&x| update.push(x.clone()));
            }
            res.push_back(first.clone());
        }
        curr_length += 1;
    }

    ChainFindResult {
        length_non_unique: non_unique,
        length_chain: curr_length,
        chain: res.into(),
        non_unique_choices: non_unique_map,
    }
}

#[cfg(test)]
mod tests {
    use std::usize;

    use crate::generator::gen::Generator;
    use crate::generator::periodic::PeriodicGen;
    use crate::group_theory::cycle::Cycle;
    use crate::group_theory::group::Group;
    use crate::group_theory::symmetric::SymmetricGroup;
    use crate::locality::chainfind::{chain_find, ChainFindResult};
    use crate::locality::reuse::calculate_lru_hits;

    #[test]
    fn s4_chain() {
        let ground = vec![1, 2, 3, 4];
        let s_m = SymmetricGroup::new(ground.len(), ground.clone());

        let hits_ranking = |cycle: &Cycle<_>| {
            let mut generator = PeriodicGen::new();
            generator.set_start(&ground.clone());
            generator.add(cycle.get_function());
            (
                calculate_lru_hits(&generator.simulate(1), 3),
                calculate_lru_hits(&generator.simulate(1), 2),
                calculate_lru_hits(&generator.simulate(1), 1),
            )
        };
        let identity = s_m.identity();
        let ChainFindResult { chain, .. } = chain_find(
            &s_m,
            identity,
            |retraversal| hits_ranking(retraversal),
            usize::MAX,
        );
        println!("{:?}", chain);
        debug_assert!(&chain.contains(&s_m.create_vec(vec![vec![]])));
        //debug_assert!(&chain.contains(&s_m.create_vec(vec![vec![3, 4]])));
        debug_assert!(&chain.contains(&s_m.create_vec(vec![vec![1, 3, 2]])));
        debug_assert!(&chain.contains(&s_m.create_vec(vec![vec![1, 4, 3, 2]])));
        //debug_assert!(&chain.contains(&s_m.create_vec(vec![vec![1, 4, 3]])));
        debug_assert!(&chain.contains(&s_m.create_vec(vec![vec![1, 4, 2, 3]])));
        debug_assert!(&chain.contains(&s_m.create_vec(vec![vec![1, 4], vec![2, 3]])));
    }
}
