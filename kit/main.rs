#[allow(unused)]
use dfmm_kit::behaviors::*;

// To run this, you can do `cargo run --bin kit` to access the binary in debug
// mode. You can also `cargo install --path kit --bin kit --force` to install
// the binary and run it with `kit`. This will be the release profile.
#[arbiter_macros::main(
    name = "DFMM Kit",
    about = "Our entrypoint to working with the DFMM Kit.",
    behaviors = Behaviors
)]
pub fn main() {}
