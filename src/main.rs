use bevy::prelude::*;
use board_plugin::resources::BoardOptions;
use bevy::log;

#[cfg(feature = "debug")]
// bevy_inspector_egui old version
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
// bevy_inspector_egui new version
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use board_plugin::BoardPlugin;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    InGame,
    Out,
}

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
        safe_start: true,
        ..Default::default()
    });
    app.init_state::<AppState>();
    app.add_plugins(BoardPlugin {
        running_state:AppState::InGame,
    });

    // app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, camera_setup);
    app.add_systems(Update, state_handler);
    // Run the app
    app.run();
}

fn camera_setup(mut commands: Commands) {
    // bevy version 0.8
    // commands.spawn_bandle(OrthographicCameraBandle::new_2d());
    // bevy version
    commands.spawn(Camera2dBundle::default());
}

fn state_handler(cur_state:ResMut<State<AppState>>,mut next_state:ResMut<NextState<AppState>>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::KeyC) {
        log::debug!("cleaning detected");
        if cur_state.get() == &AppState::InGame {
            log::info!("clearing game");
            next_state.set(AppState::Out);
        }
    }
    if keys.just_pressed(KeyCode::KeyG) {
        log::debug!("loading detected");
        if cur_state.get() == &AppState::Out {
            log::info!("loading game");
            next_state.set(AppState::InGame);
       }
    }
}
