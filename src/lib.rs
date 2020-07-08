mod utils;

mod engine;
pub use engine::Engine;
pub use engine::enums::Affiliation;
pub use engine::enums::Direction;
pub use engine::enums::Sprite;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;