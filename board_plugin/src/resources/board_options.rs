use bevy::prelude::{Resource, Vec3};
use serde::{Deserialize, Serialize};

/// Tile size options
#[derive(Debug,Clone,Serialize,Deserialize)]
pub enum TileSize {
    /// Fixed tile size
    Fixed(f32),
    /// Window adaptive tile size
    Adaptive {min: f32, max: f32}, 
}

/// Board positon customize options
#[derive(Debug,Clone,Serialize,Deserialize)]
pub enum BoardOption {
    /// Centerd board
    Centerd { offset: Vec3},
    /// Custom position
    Custom(Vec3),
}

/// Board generation options. Must be used as a resource
/// need Resource derive if this struct is resource type.
#[derive(Debug,Clone,Serialize,Deserialize, Resource)]
pub struct BoardOptions {
    /// Tile map size
    pub map_size:(u16,u16),
    /// bomb count
    pub bomb_count: u16,
    /// Board world position
    pub position:BoardOption,
    /// Tile world size
    pub tile_size: TileSize,
    /// Padding between tiles
    pub tile_padding: f32,
    /// Does the board generate a safe place to start
    pub safe_start: bool,
}

impl Default for TileSize {
    fn default() -> Self {
        Self::Adaptive {
            min: 10.0,
            max: 50.0,
        }
    }
}

impl Default for BoardOption {
    fn default() -> Self {
        Self::Centerd { 
            offset: Default::default(),
        }
    }
}

impl Default for BoardOptions {
    fn default() -> Self {
        Self {
            map_size: (15, 15),
            bomb_count: 30,
            position: Default::default(),
            tile_size: Default::default(),
            tile_padding: 0.0,
            safe_start: false,
        }
    }
}
