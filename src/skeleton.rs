use core::f32;

use crate::bone::Bone;
use macroquad::prelude::*;

pub struct Skeleton {
    origin: Vec2,
    bones: Vec<Bone>,
}

impl Skeleton {
    pub fn new(origin: Option<Vec2>) -> Self {
        Self {
            origin: origin.unwrap_or(vec2(0.0, 0.0)),
            bones: Vec::new(),
        }
    }

    pub fn add_bone(&mut self, length: f32, angle: f32) {
        let start = if let Some(last) = self.bones.last() {
            last.end()
        } else {
            self.origin
        };

        self.bones.push(Bone::new(start, length, angle));
    }

    pub fn update(&mut self, target: Vec2) {
        if self.bones.len() < 2 {
            return; // Need at least 2 bones
        }

        let l1 = self.bones[0].length;
        let l2 = self.bones[1].length;

        // Vector from origin to target
        let to_target = target - self.origin;
        let direction = to_target.normalize_or_zero();

        // Clamp distance to reachable range
        let clamped_distance = to_target.length().clamp((l1 - l2).abs(), l1 + l2);
        let actual_target = self.origin + direction * clamped_distance;
        let to_actual = actual_target - self.origin;
        let dist = to_actual.length();

        // Handle target at origin
        if dist < 0.001 {
            self.bones[0].angle = 0.0;
            self.bones[1].angle = f32::consts::PI;
        } else {
            // Law of cosines to find angles
            let cos_angle2 = (l1 * l1 + l2 * l2 - dist * dist) / (2.0 * l1 * l2);

            // Clamp to avoid NaN from acos
            let cos_angle2_clamped = cos_angle2.clamp(-1.0, 1.0);
            let angle2 = f32::acos(cos_angle2_clamped);

            // Calculate first bone angle
            let cos_angle1_part = (l1 * l1 + dist * dist - l2 * l2) / (2.0 * l1 * dist);
            let cos_angle1_part_clamped = cos_angle1_part.clamp(-1.0, 1.0);
            let angle1_part = f32::acos(cos_angle1_part_clamped);

            let target_angle = f32::atan2(to_actual.y, to_actual.x);
            let angle1 = target_angle - angle1_part;

            // Set the angles
            self.bones[0].angle = angle1;
            self.bones[1].angle = angle1 + std::f32::consts::PI - angle2;
        }

        // Update bone positions
        self.bones[0].start = self.origin;
        self.bones[1].start = self.bones[0].end();
    }

    pub fn render(&self) {
        for bone in &self.bones {
            bone.render();
        }

        // Draw circle at origin for reference
        draw_circle(self.origin.x, self.origin.y, 3.0, RED);
    }

    pub fn set_origin(&mut self, new_origin: Vec2) {
        self.origin = new_origin;

        // Recalculate bone positions
        if !self.bones.is_empty() {
            self.bones[0].start = self.origin;
            for i in 1..self.bones.len() {
                self.bones[i].start = self.bones[i - 1].end();
            }
        }
    }
}
