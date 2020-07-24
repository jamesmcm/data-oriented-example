mod dyntrait;
mod linked_list;
mod pattern_match;
mod soa;

pub use dyntrait::{gen_vec_dyn, gen_vecs, gen_vecs_box, run_dyn, run_vecs, run_vecs_box, MyTrait};
pub use linked_list::{gen_list, gen_vec, run_list, run_vec};
pub use pattern_match::{gen_mixed, gen_separate, run_mixed, run_separate};
pub use soa::{gen_dop, gen_oop, run_dop, run_oop, DOPlayers, Player};

fn main() {
    println!("Hello, world!");
}
