//! Utilities for loading and managing resources, including images, maps, and sounds.
pub mod map;

// Only ResourceKind is used from here; no need for the extra indirection.
mod resource_kind;
pub use resource::resource_kind::ResourceKind;

pub mod loading;

// Imports to run unit tests
#[cfg(test)] mod test_map;
#[cfg(test)] mod test_loading;
