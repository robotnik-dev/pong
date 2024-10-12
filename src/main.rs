use bevy::{
    color::palettes::css::{BLUE, GREEN, RED},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WorldInspectorPlugin::new()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
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
        .add_systems(Update, toggle_window_resolution)
        .add_systems(Update, (move_player).chain())
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

#[derive(Debug, Component)]
struct Ball;

#[derive(Debug, Component, Reflect)]
struct Movement {
    speed: f32,
    direction: Vec2,
}

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

    let character_width = 64.;
    let character_height = 250.;
    // spawn player
    commands.spawn((
        Player,
        Collider::cuboid(character_width / 2., character_height / 2.0),
        RigidBody::KinematicVelocityBased,
        KinematicCharacterController {
            apply_impulse_to_dynamic_bodies: true,
            ..default()
        },
        Velocity::zero(),
        Movement {
            speed: 200.0,
            direction: Vec2::ZERO,
        },
        Restitution::coefficient(1.0),
        Friction::coefficient(0.0),
        MaterialMesh2dBundle {
            mesh: meshes
                .add(Rectangle::new(character_width, character_height))
                .into(),
            material: materials.add(Color::from(BLUE)),
            // at the left side of the screen
            transform: Transform::from_xyz(character_width / 2.0, window.height() / 2.0, 0.),
            ..default()
        },
    ));

    // spawn Opponent
    commands.spawn((
        Opponent,
        Collider::cuboid(character_width / 2., character_height / 2.0),
        RigidBody::KinematicVelocityBased,
        KinematicCharacterController {
            apply_impulse_to_dynamic_bodies: true,
            ..default()
        },
        Movement {
            speed: 10.0,
            direction: Vec2::ZERO,
        },
        Velocity::zero(),
        Restitution::coefficient(1.0),
        Friction::coefficient(0.0),
        MaterialMesh2dBundle {
            mesh: meshes
                .add(Rectangle::from_size(Vec2::new(
                    character_width,
                    character_height,
                )))
                .into(),
            material: materials.add(Color::from(RED)),
            // at the right side of the screen
            transform: Transform::from_xyz(
                window.width() - character_width / 2.0,
                window.height() / 2.0,
                0.,
            ),
            ..default()
        },
    ));

    // spawn ball in the middle
    let ball_radius = 25.;
    commands.spawn((
        Ball,
        RigidBody::Dynamic,
        GravityScale(0.),
        ExternalImpulse {
            impulse: Vec2::new(1_000_000., 0.),
            torque_impulse: 0.,
        },
        Collider::ball(ball_radius),
        Restitution::coefficient(1.0),
        Friction::coefficient(0.0),
        Ccd::enabled(),
        Sleeping::disabled(),
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(ball_radius)).into(),
            material: materials.add(Color::from(GREEN)),
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.),
            ..default()
        },
    ));
}

fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    mut velocity_q: Query<(&mut Velocity, &mut Movement), With<Player>>,
) {
    let (mut velocity, mut movement) = velocity_q.single_mut();
    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
        movement.direction = Vec2::Y;
    }
    if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
        movement.direction = Vec2::NEG_Y;
    }
    velocity.linvel = movement.direction * movement.speed;
}
