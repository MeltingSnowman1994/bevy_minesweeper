use bevy::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct SpriteMaterial {
    pub color: Color,
    pub texture: Handle<Image>,
}

impl SpriteMaterial {
    pub fn default() -> Self {
        Self {
            color: Color::WHITE,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Resource)]
pub struct BoardAssets {
    /// Label
    pub label: String,
    ///
    pub board_material: SpriteMaterial,
    ///
    pub tile_material: SpriteMaterial,
    ///
    pub covered_tile_material: SpriteMaterial,
    ///
    pub bomb_counter_font: Handle<Font>,
    ///
    pub bomb_counter_colors: Vec<Color>,
    ///
    pub flag_material: SpriteMaterial,
    ///
    pub bomb_material: SpriteMaterial,
}

impl BoardAssets {
    /// Default bomb counter color set
    pub fn default_color() -> Vec<Color> {
        vec![
            Color::WHITE,
            Color::srgba(0.0, 1.0, 0.0, 1.0),
            Color::srgba(0.9, 0.9, 0.1, 1.0),
            Color::srgba(1.0, 0.8, 0.6, 1.0),
            Color::srgba(0.2, 0.1, 0.5, 1.0),
        ]
    }

    /// Safely retrieves th color matching a bomb counter
    pub fn bomb_counter_colors(&self, counter: u8) -> Color {
        let counter = counter.saturating_sub(1) as usize;
        match self.bomb_counter_colors.get(counter) {
            Some(c) => *c,
            None => match self.bomb_counter_colors.last() {
                None => Color::WHITE,
                Some(c) => *c,
            }
        }
    }
 }
