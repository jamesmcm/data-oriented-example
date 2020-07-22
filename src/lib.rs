mod pattern_match;
mod soa;

pub use pattern_match::{gen_mixed, gen_separate, run_mixed, run_separate};
pub use soa::{gen_dop, gen_oop, run_dop, run_oop, DOPlayers, Player};

fn main() {
    println!("Hello, world!");
}
