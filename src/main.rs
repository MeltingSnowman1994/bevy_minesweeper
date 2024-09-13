use bevy::prelude::*;
use board_plugin::resources::BoardOptions;

#[cfg(feature = "debug")]
// bevy_inspector_egui old version
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
// bevy_inspector_egui new version
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use board_plugin::BoardPlugin;

fn main() {
    let mut app = App::new();

    // window setup
    // bevy version 0.9
    // app.insert_resource(WindowDescriptor {
    //     title:"Mine Sweeper!".to_string(),
    //     width: 700.,
    //     height: 800.,
    //     Default::default()
    // })
    // bevy version 0.10
    let window = Window {
        title: "Mine Sweeper!".to_string(),
        resolution: (700., 800.).into(),
        ..Default::default()
    };
    let primary_window = Some(window);
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window,
        ..default()
    }));

    #[cfg(feature = "debug")]
    // debug hierarchy inspector
    app.add_plugins(WorldInspectorPlugin::new());
    // startup system
    // bevy version 0.10
    // app.add_startup_system(camera_setup)
    // bevy version 0.11
    app.insert_resource(BoardOptions{
        map_size:(20,20),
        bomb_count: 40,
        tile_padding: 3.,
        ..Default::default()
    });
    app.add_plugins(BoardPlugin);

    // app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, camera_setup);
    // Run the app
    app.run();
}

fn camera_setup(mut commands: Commands) {
    // bevy version 0.8
    // commands.spawn_bandle(OrthographicCameraBandle::new_2d());
    // bevy version
    commands.spawn(Camera2dBundle::default());
}
