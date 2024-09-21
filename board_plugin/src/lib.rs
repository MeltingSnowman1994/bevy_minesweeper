pub mod components;
pub mod resources;
mod bounds;
mod systems;
mod events;

use bevy::log;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use bevy::window::PrimaryWindow;

use components::Bomb;
use components::BombNeighbor;
use components::Coordinates;
use components::Uncover;
use resources::board::Board;
use resources::tile::Tile;
use resources::BoardOption;
use resources::TileSize;
use resources::tile_map::TileMap;
use resources::BoardOptions;
use bounds::Bounds2;
use bevy::math::Vec3Swizzles;
use crate::events::*;

pub struct BoardPlugin<T> {
    pub running_state: T,
}

impl<T: States> Plugin for BoardPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.running_state.clone()), Self::create_borad);
        // app.add_systems(Startup, Self::create_borad);
        app.add_systems(Update,( 
            systems::input::input_handling,
            systems::uncover::trigger_event_handler,
            systems::uncover::uncover_tiles,
        ).run_if(in_state(self.running_state.clone()))); 
        // app.add_systems(Update , systems::uncover::trigger_event_handler);
        // app.add_systems(Update (self.running_state), systems::uncover::uncover_tiles);
        app.add_systems(OnExit(self.running_state.clone()), Self::cleanup_board);
        app.add_event::<TileTriggerEvent>();
        log::info!("Loaded Board Plugin");
    }
}

impl<T> BoardPlugin<T> {
    pub fn create_borad(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        window: Query<&Window, With<PrimaryWindow>>,
        asset_server: Res<AssetServer>,
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
        let font = asset_server.load("fonts/pixeled.ttf");
        let bomb_image = asset_server.load("sprites/bomb.png");
        let mut covered_tiles = HashMap::with_capacity((tile_map.width() * tile_map.height()).into()); 
        let mut safe_start = None;
        let board_entity = commands
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
                Self::spawn_tile(
                    parent,
                    &tile_map,
                    tile_size,
                    options.tile_padding,
                    Color::srgba(0.3, 0.3, 0.3, 1.0),
                    bomb_image,
                    font,
Color::srgba(0.6, 0.6, 0.6, 1.0),
                    &mut covered_tiles,
                    &mut safe_start,
            )
            })
            .id();
            if options.safe_start {
                if let Some(entity) = safe_start {
                    commands.entity(entity).insert(Uncover);
                }
            }
        commands.insert_resource(Board {
            tile_map,
            bounds: Bounds2 {
                position: board_position.xy(),
                size: board_size,
            },
            tile_size,
            covered_tiles,
            entity: board_entity,
        });
    }

    fn bomb_count_text_bundle(count: u8, font: Handle<Font>, size: f32) -> Text2dBundle{
        // We ritrieve the text and the correct color
        let (text, color) = (
            count.to_string(),
            match count {
                1 => Color::WHITE,
                2 => Color::srgba(0.0, 1.0, 0.0, 1.0),
                3 => Color::srgba(0.9, 0.9, 0.1, 1.0),
                4 => Color::srgba(1.0, 0.8, 0.6, 1.0),
                _ => Color::srgba(0.2, 0.1, 0.5, 1.0),
            },
        );
        // We generate a text bundle
        Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: text,
                    style: TextStyle {
                        color,
                        font,
                        font_size: size,
                    },
                }],
                ..Default::default()
            },
            transform: Transform::from_xyz(0., 0., 1.),
            ..Default::default()
        }
    }

    fn spawn_tile(
        parent: &mut ChildBuilder,
        tile_map: &TileMap,
        size: f32,
        padding: f32,
        color: Color,
        bomb_image: Handle<Image>,
        font: Handle<Font>,
        covered_tile_color: Color,
        covered_tiles: &mut HashMap<Coordinates, Entity>,
        safe_start_entity: &mut Option<Entity>,
    ) {
        for (y, line) in tile_map.iter().enumerate(){
            for(x, tile) in line.iter().enumerate(){
                let coordinates = Coordinates {
                    x: x as u16,
                    y: (tile_map.height() - 1) - (y as u16),
                };
                let mut cmd = parent.spawn_empty();
                cmd.insert(SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::splat(size - padding)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(
                        (x as f32 * size) + (size / 2.),
                        (y as f32 * size) + (size / 2.),
                        1.,
                    ),
                    ..Default::default()
                })
                .insert(Name::new(format!("Tile ({}, {})",x, y)))
                .insert(coordinates)
                .with_children(|parent| {
                    let entity = parent
                        .spawn(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::splat(size - padding)),
                                color: covered_tile_color,
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(0., 0., 2.),
                            ..Default::default()
                        })
                        .insert(Name::new("Tile Cover"))
                        .id();
                    covered_tiles.insert(coordinates, entity);
                    if safe_start_entity.is_none() && *tile == Tile::Empty {
                        *safe_start_entity = Some(entity);
                    }
                });
                match tile {
                    Tile::Bomb => {
                        cmd.insert(Bomb);
                        cmd.with_children(|parent| {
                            parent.spawn(SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::splat(size - padding)),
                                    ..Default::default()
                                },
                                transform: Transform::from_xyz(0., 0., 1.),
                                texture: bomb_image.clone(),
                                ..Default::default()
                            });
                        });
                    }
                    Tile::BombNeighbor(v) => {
                        cmd.insert(BombNeighbor {count: *v});
                        cmd.with_children(|parent| {
                            parent.spawn(Self::bomb_count_text_bundle(
                                *v,
                                font.clone(),
                                size - padding,
                            ));
                        });
                    }
                    Tile::Empty => (),
                }
            }
        }
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

    fn cleanup_board(board: Res<Board>, mut commands: Commands) {
        commands.entity(board.entity).despawn_recursive();
        commands.remove_resource::<Board>();
    }
}
