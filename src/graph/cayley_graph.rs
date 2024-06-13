use petgraph::graph::NodeIndex;

use crate::graph::action_graph::ActionGraph;
use crate::group::cycle::Cycle;
use crate::group::group::Group;
use crate::group::symmetric::SymmetricGroup;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::fmt::Debug;

// NOTE THERE IS A SMALL DESIGN BUG !
// We are suppose to take in Group ACTIONS not just regular groups.
pub struct CayleyGraph<V> 
    where V: Clone+Copy+Hash+Eq+'static
{
    node_map: HashMap<Cycle<V>, NodeIndex>,
    action_graph: ActionGraph<V>,
}



impl<V> CayleyGraph<V> 
where 
    V: Debug+Copy+Eq+Hash+Clone+ToString+'static,
{
    pub fn new(group: &SymmetricGroup<V>) -> Self {
        let mut ag = ActionGraph::<V>::new();
        //collect nodes first
        let mut nodemap: HashMap<Cycle<V>, NodeIndex> = HashMap::new();
        for g in group.get_set().into_iter() {
            nodemap.insert(g.clone(), ag.add_node(Box::new(g.get_function()), g.display()));
        }
        //
        let generators: Vec<Cycle<V>> = group.get_generator();

        let mut elements = HashSet::new();
        //construct a bfs, with the identity element intiially in it.
        let e: Cycle<V> = group.identity();
        
        let mut q = VecDeque::from([e]);
        while !q.is_empty() {
            if let Some(element) = q.pop_front() {
                if elements.contains(&element) { continue; }
                elements.insert(element.clone());
                let a = nodemap.get(&element).unwrap();
                for gen in generators.iter() {
                    {
                        let new_element = group.op(gen.clone(), element.clone());
                        q.push_front(new_element.clone());
                        let b = nodemap.get(&new_element).unwrap();
                        ag.add_edge((*a, *b), gen.display());
                    }
                    {
                        let new_element = group.op(element.clone(), gen.clone());
                        q.push_front(new_element.clone());
                        let b = nodemap.get(&new_element).unwrap();
                        ag.add_edge((*a, *b), gen.display());
                    }
                }
            }
        }
        CayleyGraph {
            node_map: nodemap,
            action_graph: ag,
        }
    }

    pub fn get_values(&self, cycle: Cycle<V>) -> Option<&Vec<V>> {
        let ind = self.node_map.get(&cycle).unwrap();
        self.action_graph.get_values(*ind)
    }
}


mod tests {
    use crate::{graph::cayley_graph::CayleyGraph, group::{cycle::Cycle, symmetric::SymmetricGroup}};
    use crate::bimap;
    use super::ActionGraph;

    #[test]
    fn group_action() {
        let ground: Vec<i32> = vec![1, 2, 3, 4, 5];
        let group: SymmetricGroup<i32> = SymmetricGroup::new((&ground).len() as i32, ground.clone());
        
        let mut s5_group: CayleyGraph<i32> = CayleyGraph::new(&group);

        s5_group.action_graph.set_start_value(&vec![1,2,3,4,5]);

        s5_group.action_graph.graph_apply();

        debug_assert_eq!(s5_group.get_values(group.create(bimap!())), Some(&vec![1, 2, 3, 4, 5]));
        debug_assert_eq!(s5_group.get_values(group.create_vec(vec![vec![1, 2]])), Some(&vec![2, 1, 3, 4, 5]));
        debug_assert_eq!(s5_group.get_values(group.create_vec(vec![vec![1, 5], vec![2, 4]])), Some(&vec![5, 4, 3, 2, 1]));
    }
}