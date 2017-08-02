//! Provides facilities for working with tilemaps

use error::DeucalionError;
use resource::loading;
use resource::ResourceKind;
use tiled;
use sfml::graphics::{Color, Texture};

use geom::{ScreenSize, WorldSize};

/// A TileMap is roughly equivalent to a `tiled::Map`, with pre-loaded images for the tilesets.
pub struct Tilemap {
    pub dimensions: WorldSize,
    pub tile_dimensions: ScreenSize,
    pub tilesets: Vec<Tileset>,
    pub layers: Vec<tiled::Layer>,
    pub object_groups: Vec<tiled::ObjectGroup>,
    pub properties: tiled::Properties,
    pub background_color: Color,
}

// A Tileset simply associates a `tiled::Tileset` with its pre-loaded image.
pub struct Tileset {
    pub metadata: tiled::Tileset,
    pub texture: Texture,
}

impl Tilemap {
    /// Given a name, return a Tilemap corresponding to it (or not, if it doesn't exist)
    pub fn by_name(name: &str) -> Result<Tilemap, DeucalionError> {
        // Get the reader and path for the map's file
        let (map_reader, mut map_path) =
            loading::get_resource_reader_and_path_by_name(ResourceKind::Map, name)?;
        let map = tiled::parse(map_reader)?;
        info!(
            "Successfully loaded a map '{}' from its TMX file at '{}'",
            name,
            map_path.to_string_lossy()
        );

        // Load the tilesets used by this map. First, figure out the directory they're in...
        map_path.pop();
        // Then load them all.
        let mut tilesets: Vec<Tileset> = Vec::with_capacity(map.tilesets.len());
        for ts in map.tilesets {
            // Determine the file path and load the file
            map_path.push(&ts.images[0].source);
            let image = Texture::from_file(&map_path.to_string_lossy());
            map_path.pop();

            // Deal with the loaded file
            if let Some(image) = image {
                info!(
                    "Successfully loaded a tileset image '{}' for map '{}'",
                    &ts.images[0].source,
                    name
                );
                tilesets.push(Tileset {
                    metadata: ts,
                    texture: image,
                });
            } else {
                info!(
                    "Failed to load a tileset image '{}' for map '{}', returning last OS error.",
                    &ts.images[0].source,
                    name
                );
                return Err(DeucalionError::IoError(::std::io::Error::last_os_error()));
            }
        }

        // Add the appropriate types to the dimensions.
        let dimensions: WorldSize = WorldSize::new(
            (map.width * map.tile_width) as f32,
            (map.height * map.tile_height) as f32,
        );
        let tile_dimensions = ScreenSize::new(map.tile_width, map.tile_height);

        // Default to a black background
        let bgcolor: Color = if let Some(rgb_color) = map.background_colour {
            Color::rgb(rgb_color.red, rgb_color.green, rgb_color.blue)
        } else {
            Color::black()
        };

        return Ok(Tilemap {
            dimensions: dimensions,
            tile_dimensions: tile_dimensions,
            tilesets: tilesets,
            layers: map.layers,
            object_groups: map.object_groups,
            properties: map.properties,
            background_color: bgcolor,
        });
    }
}
