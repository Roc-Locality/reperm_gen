use reperm_gen::group::symmetric::{sym, SymmetricGroup};
use reperm_gen::group::group::Group;
use reperm_gen::group::cycle::Cycle;
use reperm_gen::bimap;



fn main() {
    let group: SymmetricGroup<i32> = sym(5);

    let nat: Cycle<i32> = Cycle::new(bimap! {1 => 2, 2 => 3, 3 => 4, 4 => 1}, group.get_ground().clone());
    let rev: Cycle<i32> = Cycle::new(bimap! {4 => 3, 3 => 2, 2 => 1, 1 => 4}, group.get_ground().clone());
    let symmetric_set = group.get_set();
    println!("{:?}", symmetric_set);
    println!("{:?}", symmetric_set.len());
}
