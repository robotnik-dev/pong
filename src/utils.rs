use bevy::math::Vec3;

use bevy::{prelude::*, render::primitives::Aabb};

pub fn get_random_direction_v3() -> Vec3 {
    Vec3::new(1., 0., 0.)
}

/// Axis Aligned Bounding Box collision check between two boxes.
/// A collision happens if four conditions are met:
/// 1. Left edge x value of box 1 is less than right edge x value of box 2
/// 2. right edge x value of box 1 is greater than left edge x value of box 2
/// 3. top edge y value of box 1 is greater than the bottom y value of box 2
/// 4. bottom edge y value of box 1 is less than the top edge y value of box 2
pub fn aabb_collision(
    box1_center: Vec3,
    box1_size: Vec3,
    box2_center: Vec3,
    box2_size: Vec3,
) -> bool {
    (box1_center - box1_size / 2.).x < (box2_center + box2_size / 2.).x
        && (box1_center + box1_size / 2.).x > (box2_center - box2_size / 2.).x
        && (box1_center + box1_size / 2.).y > (box2_center - box2_size / 2.).y
        && (box1_center - box1_size / 2.).y < (box2_center + box2_size / 2.).y
}
