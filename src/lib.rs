pub mod group_theory {
    pub mod cycle;
    pub mod group;
    pub mod symmetric;
}

pub mod generator {
    pub mod gen;
    pub mod iterative;
    pub mod periodic;
}

pub mod graph {
    pub mod action_graph;
    pub mod cayley_graph;
}

pub mod math {
    pub mod combinations;
}
pub mod locality {
    pub mod chainfind;
    pub mod reuse;
}

pub mod macros;
pub use ::bimap;
