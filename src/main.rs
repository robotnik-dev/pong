use bevy::{
    color::palettes::css::{BLUE, GREEN, RED},
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    utils::info,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod utils;
use utils::{aabb_collision, get_random_direction_v3};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WorldInspectorPlugin::new()))
        .insert_resource(ResolutionSettings {
            small: Vec2 { x: 640., y: 360. },
            medium: Vec2 { x: 800., y: 600. },
            large: Vec2 { x: 1920., y: 1080. },
        })
        .register_type::<Movement>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                toggle_window_resolution,
                move_player,
                move_ball,
                bounce_ball,
            ),
        )
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
    direction: Option<Vec3>,
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
        Movement {
            speed: 3.0,
            direction: None,
        },
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
        Movement {
            speed: 2.5,
            direction: None,
        },
        MaterialMesh2dBundle {
            mesh: meshes
                .add(Rectangle::new(character_width, character_height))
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
    commands.spawn((
        Ball,
        Movement {
            speed: 6.,
            direction: Some(get_random_direction_v3()),
        },
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(25.)).into(),
            material: materials.add(Color::from(GREEN)),
            // at the right side of the screen
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.),
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

fn move_ball(mut q_ball: Query<(&mut Transform, &Movement), With<Ball>>) {
    let (mut transform, movement) = q_ball.single_mut();
    if let Some(direction) = movement.direction {
        transform.translation += direction * movement.speed;
    }
}

/// If some collision happens between the ball and some other object (including screen bounds), redirect the balls direction without changing the velocity
fn bounce_ball(
    mut q_ball: Query<(&mut Movement, &Transform, &Mesh2dHandle), With<Ball>>,
    q_objects: Query<(&Transform, &Mesh2dHandle), Without<Ball>>,
    meshes: Res<Assets<Mesh>>,
    mut q_window: Query<&Window>,
) {
    let (mut ball_movement, ball_transform, ball_handle) = q_ball.single_mut();
    let Some(ball_mesh) = meshes.get(ball_handle.id()) else {
        return;
    };
    let Some(aabb_ball) = ball_mesh.compute_aabb() else {
        return;
    };

    // check screen collision
    let mut impact_point = Vec3::ZERO;
    let window = q_window.single_mut();
    if (ball_transform.translation.x - aabb_ball.half_extents.x) <= 0. {
        // left side
        impact_point = Vec3::new(
            ball_transform.translation.x - aabb_ball.half_extents.x,
            ball_transform.translation.y,
            0.,
        );
    } else if (ball_transform.translation.y + aabb_ball.half_extents.y) >= window.height() {
        // top side
        impact_point = Vec3::new(
            ball_transform.translation.x,
            ball_transform.translation.y + aabb_ball.half_extents.y,
            0.,
        );
    } else if (ball_transform.translation.x + aabb_ball.half_extents.x) >= window.width() {
        // right side
        impact_point = Vec3::new(
            ball_transform.translation.x + aabb_ball.half_extents.x,
            ball_transform.translation.y,
            0.,
        );
    } else if (ball_transform.translation.y - aabb_ball.half_extents.y) <= 0. {
        // bottom side
        impact_point = Vec3::new(
            ball_transform.translation.x,
            ball_transform.translation.y - aabb_ball.half_extents.y,
            0.,
        );
    }

    // check object collision
    for (object_transform, handle) in q_objects.iter() {
        if let Some(object_mesh) = meshes.get(handle.id()) {
            let Some(aabb_object) = object_mesh.compute_aabb() else {
                return;
            };
            if aabb_collision(
                ball_transform.translation,
                (aabb_ball.half_extents * 2.).into(),
                object_transform.translation,
                (aabb_object.half_extents * 2.).into(),
            ) {
                // TODO: compute impact point
                // impact point is the vector pointing from the objects center to the balls center, cut in length by the half extends of the ball
                impact_point = object_transform.translation
                    + (object_transform.translation
                        - ball_transform.translation
                        - Vec3::splat(25.));
            }
        }
    }

    if impact_point != Vec3::ZERO {
        // change direction of the ball
        if let Some(direction) = ball_movement.direction {
            let normal = (impact_point - ball_transform.translation).normalize();
            ball_movement.direction = Some(direction - 2.0 * direction.dot(normal) * normal);
        }
    }
}
