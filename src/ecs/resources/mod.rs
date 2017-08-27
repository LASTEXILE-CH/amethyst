//! Resources that can be added to `ecs::World`.
//!
//! `Camera`, `ScreenDimensions`, and `Time` are added by default and
//! automatically updated every frame by `Application`.

// pub use self::camera::{Camera, Projection};
// pub use self::screen_dimensions::ScreenDimensions;
// pub use self::time::Time;

// mod camera;
// mod screen_dimensions;
// mod time;

pub use self::rendering::{Factory, FactoryFuture, MeshFuture, AmbientColor};

pub mod input;

mod rendering;