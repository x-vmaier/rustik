use core::f32;
use macroquad::prelude::*;

use crate::bone::Bone;

/// A 2-bone skeleton for inverse kinematics.
///
/// The skeleton is rooted at an origin and contains up to two connected bones.
/// <br> Supports both single-bone pointing and two-bone analytical IK.
///
/// # Examples
///
/// ```
/// let mut skeleton = Skeleton::new(vec2(100.0, 100.0));
/// skeleton.add_bone(80.0, 0.0);
/// skeleton.add_bone(80.0, 0.0);
///
/// // Update to point toward target
/// skeleton.update(vec2(200.0, 150.0));
/// ```
#[derive(Debug, Clone)]
pub struct Skeleton {
    origin: Vec2,
    bones: Vec<Bone>,
}

#[allow(dead_code)]
impl Skeleton {
    pub fn new(origin: Vec2) -> Self {
        Self {
            origin,
            bones: Vec::with_capacity(2),
        }
    }

    /// Adds a bone to the skeleton.
    pub fn add_bone(&mut self, length: f32, angle: f32) {
        if self.bones.len() >= 2 || length <= 0.0 {
            return;
        }

        let start = self.end_effector();
        self.bones.push(Bone::new(start, length, angle));
    }

    /// Updates the skeleton to reach toward the target position.
    pub fn update(&mut self, target: Vec2) {
        match self.bones.len() {
            0 => {}
            1 => self.update_single_bone(target),
            2 => self.update_two_bone_ik(target),
            _ => unreachable!("Skeleton cannot have more than 2 bones"),
        }
    }

    /// Returns the number of bones in the skeleton.
    pub fn bone_count(&self) -> usize {
        self.bones.len()
    }

    /// Returns the skeleton's origin point.
    pub fn origin(&self) -> Vec2 {
        self.origin
    }

    /// Returns the end effector position.
    pub fn end_effector(&self) -> Vec2 {
        self.bones
            .last()
            .map(|bone| bone.end())
            .unwrap_or(self.origin)
    }

    /// Returns the total reach of the skeleton.
    pub fn max_reach(&self) -> f32 {
        self.bones.iter().map(|bone| bone.length()).sum()
    }

    /// Sets a new origin for the skeleton and updates bone positions.
    pub fn set_origin(&mut self, new_origin: Vec2) {
        self.origin = new_origin;
        self.update_bone_positions();
    }

    /// Renders the skeleton using macroquad drawing functions.
    pub fn render(&self) {
        // Draw bones
        for bone in &self.bones {
            bone.render();
        }

        // Draw origin marker
        draw_circle(self.origin.x, self.origin.y, 4.0, RED);
    }

    fn update_single_bone(&mut self, target: Vec2) {
        let to_target = target - self.origin;
        if to_target.length() > 0.001 {
            self.bones[0].set_angle(to_target.y.atan2(to_target.x));
        }
        self.bones[0].set_start(self.origin);
    }

    fn update_two_bone_ik(&mut self, target: Vec2) {
        let l1 = self.bones[0].length();
        let l2 = self.bones[1].length();

        // Vector from origin to target
        let to_target = target - self.origin;
        let direction = to_target.normalize_or_zero();
        let distance = to_target.length();

        // Calculate reachable target
        let clamped_distance = distance.clamp((l1 - l2).abs(), l1 + l2);
        let actual_target = self.origin + direction * clamped_distance;
        let to_actual = actual_target - self.origin;
        let dist = to_actual.length();

        if dist < f32::EPSILON {
            // Target at origin
            self.bones[0].set_angle(0.0);
            self.bones[1].set_angle(f32::consts::PI);
        } else {
            // Law of cosines for IK
            let cos_interior_angle = (l1 * l1 + l2 * l2 - dist * dist) / (2.0 * l1 * l2);
            let interior_angle = cos_interior_angle.clamp(-1.0, 1.0).acos();

            let cos_first_part = (l1 * l1 + dist * dist - l2 * l2) / (2.0 * l1 * dist);
            let first_part_angle = cos_first_part.clamp(-1.0, 1.0).acos();

            let target_angle = to_actual.y.atan2(to_actual.x);
            let first_bone_angle = target_angle - first_part_angle;
            let second_bone_angle = first_bone_angle + f32::consts::PI - interior_angle;

            self.bones[0].set_angle(first_bone_angle);
            self.bones[1].set_angle(second_bone_angle);
        }

        self.update_bone_positions();
    }

    fn update_bone_positions(&mut self) {
        if self.bones.is_empty() {
            return;
        }

        self.bones[0].set_start(self.origin);
        if self.bones.len() > 1 {
            let second_start = self.bones[0].end();
            self.bones[1].set_start(second_start);
        }
    }
}
