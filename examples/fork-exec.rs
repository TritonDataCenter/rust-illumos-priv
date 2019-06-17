use illumos_priv::{PrivOp, PrivPtype, PrivSet, Privilege};
use std::process::Command;

fn main() {
    let pset = PrivSet::new_basic().unwrap();

    pset.delset(Privilege::ProcExec).unwrap();
    pset.delset(Privilege::ProcFork).unwrap();

    illumos_priv::setppriv(PrivOp::Set, PrivPtype::Permitted, &pset).unwrap();
    illumos_priv::setppriv(PrivOp::Set, PrivPtype::Effective, &pset).unwrap();

    match Command::new("ls").output() {
        Err(e) => eprintln!("failed to fork/exec ls: {:?}", e.kind()),
        Ok(_) => panic!("shouldn't be able to fork/exec"),
    }
}
