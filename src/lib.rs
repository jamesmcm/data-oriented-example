mod linked_list;
mod pattern_match;
mod soa;

pub use linked_list::{gen_list, gen_vec, run_list, run_vec};
pub use pattern_match::{gen_mixed, gen_separate, run_mixed, run_separate};
pub use soa::{gen_dop, gen_oop, run_dop, run_oop, DOPlayers, Player};
fn main() {
    println!("Hello, world!");
}
