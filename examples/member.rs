use illumos_priv::{PrivSet, Privilege};

fn main() {
    let pset = PrivSet::new_basic().unwrap();

    println!(
        "{:?} is in the PrivSet: {}",
        Privilege::ProcFork,
        pset.is_member(Privilege::ProcFork)
    );

    pset.delset(Privilege::ProcFork).unwrap();

    println!(
        "{:?} is in the PrivSet: {}",
        Privilege::ProcFork,
        pset.is_member(Privilege::ProcFork)
    );
}
