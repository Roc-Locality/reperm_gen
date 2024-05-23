mod group {
    pub mod symmetric;
    pub mod group;
    pub mod cycle;
}

use bimap::BiMap;
use crate::group::symmetric::SymmetricGroup;
use crate::group::group::Group;
use crate::group::cycle::Cycle;

macro_rules! bimap {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {{
        let mut map = BiMap::new();
        $(map.insert($k, $v);)*
        map
    }};
}

fn main() {
    let ground: Vec<u32> = [1, 2, 3, 4].into_iter().collect();
    println!("ground = {:?}", ground);
    let group: SymmetricGroup<u32> = SymmetricGroup::new(4, ground.clone());

    let nat: Cycle<u32> = Cycle::new(bimap! {1 => 2, 2 => 3, 3 => 4, 4 => 1}, ground.clone());
    let rev: Cycle<u32> = Cycle::new(bimap! {4 => 3, 3 => 2, 2 => 1, 1 => 4}, ground.clone());

    println!("nat = {:?}", nat);
    println!("rev = {:?}", rev);
    //println!("a * b = {:?}", id * rev);
    //println!("rev * nat = {:?}", rev * nat);
    println!("(b * a)(1) = {:?}", (rev * nat).eval(1));
    println!("generator {:?}", group.get_generator());
    println!("Hello, world!");
}
