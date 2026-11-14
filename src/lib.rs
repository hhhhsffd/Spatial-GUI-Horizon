#[path = "../mixins/spatial_gui/config.rs"]
pub mod config;
#[path = "../mixins/spatial_gui/geometry.rs"]
pub mod geometry;

pub use config as spatial_config;
pub use geometry as spatial_surface;
#[path = "../mixins/spatial_gui/avatar.rs"]
pub mod avatar;
#[path = "../mixins/spatial_gui/editor.rs"]
pub mod editor;
#[path = "../mixins/spatial_gui/motion.rs"]
pub mod motion;
