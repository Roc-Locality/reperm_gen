pub mod group {
    pub mod symmetric;
    pub mod group;
    pub mod cycle;
}

pub mod generator {
    pub mod generator;
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
    pub mod reuse;
    pub mod chainfind;
}

pub mod macros;
pub use ::bimap;