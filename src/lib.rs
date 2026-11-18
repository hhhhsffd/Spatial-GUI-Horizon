#[path = "native/config.rs"]
pub mod config;
#[path = "native/geometry.rs"]
pub mod geometry;

pub use config as spatial_config;
pub use geometry as spatial_surface;
#[path = "native/avatar.rs"]
pub mod avatar;
#[path = "native/editor.rs"]
pub mod editor;
#[path = "native/motion.rs"]
pub mod motion;

// Native-route support code is compiled into the game by the source-build helper.
// The flat package also exposes the factory required by the unified native format.
#[derive(Default)]
struct SpatialGui;

#[mod_api::horizon_mod]
impl mod_api::Hooks for SpatialGui {}
