use dfmm_kit::behaviors::*;

#[arbiter_macros::main(
    name = "DFMM Kit",
    about = "Our entrypoint to working with the DFMM Kit.",
    behaviors = Behaviors
)]
pub async fn main() {}
