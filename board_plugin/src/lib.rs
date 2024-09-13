pub mod components;
pub mod resources;

use bevy::log;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use components::Coordinates;
use resources::BoardOption;
use resources::TileSize;
use resources::tile_map::TileMap;
use resources::BoardOptions;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::create_borad);
        log::info!("Loaded Board Plugin");
    }
}

impl BoardPlugin {
    pub fn create_borad(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        window: Query<&Window, With<PrimaryWindow>>,
    ){
        let options = match board_options {
            Some(o) => o.clone(),
            None => todo!(),
        };
        let mut tile_map = TileMap::empty(options.map_size.0, options.map_size.1);
        tile_map.set_bombs(options.bomb_count);
        #[cfg(feature = "debug")]
        log::info!("{}",tile_map.console_output());

        let tile_size = match options.tile_size {
            TileSize::Fixed(v) => v,
            TileSize::Adaptive { min, max } => Self::adaptive_tile_size(
                window, 
                (min, max),
                (tile_map.width(), tile_map.height()),
            ),
        };

        let board_size = Vec2::new(
            tile_map.width() as f32 * tile_size,
            tile_map.height() as f32 * tile_size,
        );
        log::info!("board size:{}", board_size);
        let board_position = match  options.position {
            BoardOption::Centerd { offset } => {
                Vec3::new(-(board_size.x / 2.), -(board_size.y / 2.), 0.) + offset
            }
            BoardOption::Custom(p) => p,
        };
        commands
            .spawn_empty()
            .insert(Name::new("Board"))
            .insert(Transform::from_translation(board_position))
            .insert(GlobalTransform::default())
            .with_children(|parent|{
                parent
                    .spawn(SpriteBundle{
                        sprite:Sprite {
                            color: Color::WHITE,
                            custom_size: Some(board_size),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(board_size.x / 2., board_size.y / 2., 0.),
                        ..Default::default()
                    })
                    .insert(Name::new("Background"));
                for (y, line) in tile_map.iter().enumerate() {
                    for (x, tile) in line.iter().enumerate() {
                        parent
                            .spawn(SpriteBundle {
                                sprite: Sprite {
                                    color: Color::hsv(0., 1., 1.),
                                    custom_size: Some(Vec2::splat(
                                        tile_size - options.tile_padding as f32,
                                    )),
                                    ..Default::default()
                                },
                                transform: Transform::from_xyz(
                                    (x as f32 * tile_size) + (tile_size / 2.),
                                    (y as f32 * tile_size) + (tile_size / 2.),
                                    1.),
                                    ..Default::default()
                            })
                            .insert(Name::new(format!("Tile ({}, {})",x,y)))
                            .insert(Coordinates {
                                x: x as u16,
                                y: y as u16,
                            });
                    }
                }
                });
    }

    fn adaptive_tile_size (
        q_window: Query<&Window, With<PrimaryWindow>>,
        (min, max): (f32, f32),
        (width, height): (u16, u16),
    ) -> f32{
        let window = q_window.single();
        let max_width = window.width() / width as f32;
        let max_height = window.height() / height as f32;
        max_width.min(max_height).clamp(min, max)
    }
}
