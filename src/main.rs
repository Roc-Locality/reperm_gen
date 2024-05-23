mod group {
    pub mod symmetric;
    pub mod group;
    pub mod cycle;
}

use std::collections::HashSet;
use crate::group::symmetric::SymmetricGroup;
use crate::group::group::Group;
use crate::group::cycle::Cycle;

fn main() {
    let ground: Vec<u32> = [1, 2, 3, 4].into_iter().collect();
    println!("ground = {:?}", ground);
    let group: SymmetricGroup<u32> = SymmetricGroup::new(4, ground.clone());

    let nat: Cycle<u32> = Cycle::new(vec![1, 2, 3, 4], ground.clone());
    let rev: Cycle<u32> = Cycle::new(vec![4, 3, 2, 1], ground.clone());

    println!("nat = {:?}", nat);
    println!("rev = {:?}", rev);
    //println!("a * b = {:?}", id * rev);
    //println!("rev * nat = {:?}", rev * nat);
    println!("(b * a)(1) = {:?}", (rev * nat).eval(1));
    println!("generator {:?}", group.get_generator());
    println!("Hello, world!");
}
