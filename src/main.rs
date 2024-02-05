#![warn(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::undocumented_unsafe_blocks)]
#![allow(unknown_lints)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused_qualifications)]
#![warn(variant_size_differences)]

mod types;
use crate::types::*;

fn main() {
    let election = Election {
        election_id: 1,
        title: "Election 2020".to_string(),
        election_mode: ElectionModeType::RealMode,
        status: ElectionStatus::Draft,
    };
    println!("{:#?}", election);
}
