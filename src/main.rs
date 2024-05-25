use reperm_gen::group::symmetric::SymmetricGroup;
use reperm_gen::group::group::Group;
use reperm_gen::group::cycle::Cycle;
use reperm_gen::bimap;



fn main() {
    let ground: Vec<u32> = [1, 2, 3].into_iter().collect();
    println!("ground = {:?}", ground);
    let group: SymmetricGroup<u32> = SymmetricGroup::new((&ground).len() as i32, ground.clone());

    let nat: Cycle<u32> = Cycle::new(bimap! {1 => 2, 2 => 3, 3 => 4, 4 => 1}, ground.clone());
    let rev: Cycle<u32> = Cycle::new(bimap! {4 => 3, 3 => 2, 2 => 1, 1 => 4}, ground.clone());
    let symmetric_set = group.get_set();
    println!("{:?}", symmetric_set);
    println!("{:?}", symmetric_set.len());
}
