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

pub mod reuse;

pub mod macros;
pub use ::bimap;