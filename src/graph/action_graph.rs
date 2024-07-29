use std::collections::HashMap;
use petgraph::{graph::{Graph, NodeIndex, EdgeIndex}, Undirected};

//should this be &'a T ?
pub type Evaluate<T> = dyn Fn(T) -> T;

/// This is an action graph which consider of nodes that can be used to compute values.
/// T is the actual functions/transformations stored, and V are the input to those functions in T.
pub struct ActionGraph<V> 
where 
    V: Clone,
{
    backing_graph: Graph<String, String, Undirected>,
    node_map: HashMap<NodeIndex, Box<Evaluate<V>>>,
    //todo: it should probably be a general iterator, not Vec<V>
    value_map: HashMap<NodeIndex, Vec<V>>,
}

impl<V> Default for ActionGraph<V>
where 
     V: Clone+'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<V> ActionGraph<V> 
where 
    V: Clone+'static,
{
    pub fn new() -> Self {
        ActionGraph {
            backing_graph: Graph::new_undirected(),
            node_map: HashMap::new(),
            value_map: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, func: Box<Evaluate<V>>, func_name: String) -> NodeIndex {
        let ind = self.backing_graph.add_node(func_name);
        self.node_map.insert(ind, func);
        ind
    }

    pub fn add_edge(&mut self, edges: (NodeIndex, NodeIndex), edge_name: String) -> EdgeIndex {
        let node1 = edges.0;
        let node2 = edges.1;
        self.backing_graph.add_edge(node1, node2, edge_name)
    }

    pub fn get_values(&self, ind: NodeIndex) -> Option<&Vec<V>> {
        self.value_map.get(&ind)
    }

    pub fn set_start_value(&mut self, values: &Vec<V>) {
        for ind in self.backing_graph.node_indices() {
            self.value_map.insert(ind, values.to_owned());
        }
    }
    /// This runs all of the computations in each of the nodes.
    pub fn graph_apply(&mut self) { 
        for node in self.backing_graph.node_indices() {
            let curr_val = self.value_map.get(&node).unwrap();
            let curr_func = self.node_map.get(&node).unwrap();
            let new_values = curr_val.iter()
                .map(|v| curr_func(v.clone()))
                .collect();
            self.value_map.insert(node, new_values);
        }
    }

}

mod tests {
    #[allow(unused_imports)]
    use super::ActionGraph;

    #[test]
    fn add_action() {
        let mut add_graph = ActionGraph::<i32>::new();
        let add_1 = move |x| x + 1;
        let add_2 = move |x| x + 2;
        let add_3 = move |x| x + 3;
        let a1 = add_graph.add_node(Box::new(add_1), String::from("add_1"));
        let a2 = add_graph.add_node(Box::new(add_2), String::from("add_2"));
        let a3 = add_graph.add_node(Box::new(add_3), String::from("add_3"));

        add_graph.add_edge((a1, a2), String::from("+1"));
        add_graph.add_edge((a2, a3), String::from("+1"));

        add_graph.set_start_value(&vec![1,2,3,4,5]);

        add_graph.graph_apply();
        debug_assert_eq!(add_graph.get_values(a1), Some(&vec![2,3,4,5,6]));
        debug_assert_eq!(add_graph.get_values(a2), Some(&vec![3,4,5,6,7]));
        debug_assert_eq!(add_graph.get_values(a3), Some(&vec![4,5,6,7,8]));
        
        add_graph.graph_apply();
        debug_assert_eq!(add_graph.get_values(a1), Some(&vec![3,4,5,6,7]));
    }
}