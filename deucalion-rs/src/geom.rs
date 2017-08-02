use euclid;

/// Marks a point as existing in the screen's concrete space.
pub struct ScreenSpace;
/// A concrete, integer point on the screen - a pixel.
pub type ScreenPoint = euclid::TypedPoint2D<u32, ScreenSpace>;
/// Indicates the concrete size of a section of the screen.
pub type ScreenSize = euclid::TypedSize2D<u32, ScreenSpace>;

/// Marks a point as existing in in-game float space.
pub struct WorldSpace;
/// A point in the game's world space.
pub type WorldPoint = euclid::TypedPoint2D<f32, WorldSpace>;
/// Indicates the size of a section of the world space.
pub type WorldSize = euclid::TypedSize2D<f32, WorldSpace>;
