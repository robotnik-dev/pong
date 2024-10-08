use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WorldInspectorPlugin::new()))
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component, Debug)]
struct Player;

#[derive(Component, Debug)]
struct Opponent;

fn setup(mut commands: Commands) {
    // spawn camera
    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: Vec3::splat(19.0),
            ..default()
        },
        ..default()
    });

    // spawn player
    commands.spawn((Player, Transform::from_xyz(0., 0.5, 0.)));

    // spawn Opponent
    commands.spawn((Opponent, Transform::from_xyz(1.0, 0.5, 0.)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_initialized() {
        // Setup app
        let mut app = App::new();

        // Add setup system
        app.add_systems(Startup, setup);

        // Run system
        app.update();

        // Check if player was spawned
        assert_eq!(
            app.world_mut().query::<&Player>().iter(app.world()).len(),
            1
        );
    }
}
