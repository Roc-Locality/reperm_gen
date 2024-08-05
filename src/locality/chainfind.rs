use std::cmp::min;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::hash::Hash;

use crate::group_theory::cycle::Cycle;
use crate::group_theory::group::Group;
use crate::group_theory::symmetric::SymmetricGroup;

#[allow(unused)]
pub fn chain_find<V, F, O>(
    group: &SymmetricGroup<V>,
    start: Cycle<V>,
    locality_calc: F,
    maxlen: usize,
) -> Vec<Cycle<V>>
where
    V: Clone + Copy + Hash + Eq + PartialEq + Debug + PartialOrd + ToString,
    F: Fn(&Cycle<V>) -> O,
    O: PartialOrd + PartialEq + Ord,
{
    let generators = group.get_generator();
    let mut res = VecDeque::new();
    res.push_back(start.clone());
    let mut curr_length: usize = start.inversions();
    let max_length = min(
        (group.ground_size() * (group.ground_size() + 1)) / 2,
        maxlen,
    );
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
            .filter(|x| locality_calc(x) > locality_calc(node))
            .collect::<Vec<_>>();
        max_locality.sort_unstable_by_key(|a| locality_calc(a));
        max_locality.dedup();
        let max_locality = max_locality;
        if let Some(&first) = max_locality.first() {
            if max_locality.len() > 1
                && locality_calc(first) == locality_calc(max_locality.get(1).unwrap())
            {
                println!("{:?} does not have a unique key!", first.display());
                max_locality
                    .iter()
                    .filter(|x| locality_calc(x) == locality_calc(first))
                    .for_each(|x| println!("\tequivalent: {}", x.display()));
            }
            res.push_back(first.clone());
        } else {
            break;
        }

        curr_length += 1
    }

    res.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use std::usize;

    use crate::generator::gen::Generator;
    use crate::generator::periodic::PeriodicGen;
    use crate::group_theory::cycle::Cycle;
    use crate::group_theory::group::Group;
    use crate::group_theory::symmetric::SymmetricGroup;
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
            (
                calculate_lru_hits(&generator.simulate(1), 3),
                calculate_lru_hits(&generator.simulate(1), 2),
                calculate_lru_hits(&generator.simulate(1), 1),
            )
        };
        let identity = s_m.identity();
        let chain: Vec<Cycle<i32>> = chain_find(
            &s_m,
            identity,
            |retraversal| hits_ranking(retraversal),
            usize::MAX,
        );
        debug_assert_eq!(
            &chain,
            &[
                s_m.create_vec(vec![vec![]]),
                s_m.create_vec(vec![vec![3, 4]]),
                s_m.create_vec(vec![vec![2, 3, 4]]),
                s_m.create_vec(vec![vec![1, 2, 3, 4]]),
                s_m.create_vec(vec![vec![1, 2, 4]]),
                s_m.create_vec(vec![vec![1, 3, 2, 4]]),
                s_m.create_vec(vec![vec![1, 4], vec![2, 3]]),
            ]
        );
    }
}
