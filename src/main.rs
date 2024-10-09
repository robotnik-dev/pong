use bevy::{
    color::palettes::css::{BLUE, GREEN, RED},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WorldInspectorPlugin::new()))
        .insert_resource(ResolutionSettings {
            small: Vec2 { x: 640.0, y: 360.0 },
            medium: Vec2 { x: 800.0, y: 600.0 },
            large: Vec2 {
                x: 1920.0,
                y: 1080.0,
            },
        })
        .register_type::<Movement>()
        .add_systems(Startup, setup)
        .add_systems(Update, (toggle_window_resolution, move_player))
        .run();
}

#[derive(Debug, Resource)]
struct ResolutionSettings {
    small: Vec2,
    medium: Vec2,
    large: Vec2,
}

#[derive(Component, Debug)]
struct Player;

#[derive(Component, Debug)]
struct Opponent;

#[derive(Debug, Component, Reflect)]
struct Movement {
    speed: f32,
}

#[derive(Debug, Component)]
struct Ball;

fn toggle_window_resolution(
    // HACK: change keys into proper settings UI
    keys: Res<ButtonInput<KeyCode>>,
    mut windows: Query<&mut Window>,
    resolution: Res<ResolutionSettings>,
) {
    let mut window = windows.single_mut();

    if keys.just_pressed(KeyCode::Digit1) {
        let res = resolution.small;
        window.resolution.set(res.x, res.y);
    }
    if keys.just_pressed(KeyCode::Digit2) {
        let res = resolution.medium;
        window.resolution.set(res.x, res.y);
    }
    if keys.just_pressed(KeyCode::Digit3) {
        let res = resolution.large;
        window.resolution.set(res.x, res.y);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: Query<&mut Window>,
    resolution: Res<ResolutionSettings>,
) {
    // set window resolution to full HD
    let mut window = windows.single_mut();
    let res = resolution.large;
    window.resolution.set(res.x, res.y);

    // spawn camera
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.),
        ..default()
    });

    // spawn player
    let character_width = 64.;
    let character_height = 250.;
    commands.spawn((
        Player,
        Movement { speed: 3.0 },
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::default()).into(),
            material: materials.add(Color::from(BLUE)),
            // at the left side of the screen
            transform: Transform::from_xyz(character_width / 2.0, window.height() / 2.0, 0.)
                .with_scale(Vec3::new(character_width, character_height, 0.)),
            ..default()
        },
    ));

    // spawn Opponent
    commands.spawn((
        Opponent,
        Movement { speed: 2.5 },
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::default()).into(),
            material: materials.add(Color::from(RED)),
            // at the right side of the screen
            transform: Transform::from_xyz(
                window.width() - character_width / 2.0,
                window.height() / 2.0,
                0.,
            )
            .with_scale(Vec3::new(character_width, character_height, 0.)),
            ..default()
        },
    ));

    // spawn ball in the middle
    commands.spawn((
        Ball,
        Movement { speed: 10. },
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::default()).into(),
            material: materials.add(Color::from(GREEN)),
            // at the right side of the screen
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.)
                .with_scale(Vec3::new(50., 50., 0.)),
            ..default()
        },
    ));
}

fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_q: Query<(&mut Transform, &Movement), With<Player>>,
) {
    let (mut player, movement) = player_q.single_mut();
    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
        player.translation.y += movement.speed;
    }
    if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
        player.translation.y -= movement.speed;
    }
}

fn keep_character_in_screen(
    window: Query<&Window>,
    mut character_q: Query<&mut Transform, With<Player>>,
) {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_moved() {
        let mut app = App::new();

        app.add_systems(Update, move_player);
    }
}
