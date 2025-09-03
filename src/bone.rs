use macroquad::prelude::*;

/// A single bone in a kinematic chain.
///
/// A bone is defined by its start position, length, and angle.
/// <br> The end position is computed from these properties.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bone {
    start: Vec2,
    length: f32,
    angle: f32,
}

#[allow(dead_code)]
impl Bone {
    pub fn new(start: Vec2, length: f32, angle: f32) -> Self {
        Self {
            start,
            length,
            angle,
        }
    }

    /// Returns the end position of the bone.
    pub fn end(&self) -> Vec2 {
        self.start
            + Vec2::new(
                self.length * self.angle.cos(),
                self.length * self.angle.sin(),
            )
    }

    /// Returns the start position of the bone.
    pub fn start(&self) -> Vec2 {
        self.start
    }

    /// Sets the start position of the bone.
    pub fn set_start(&mut self, start: Vec2) {
        self.start = start;
    }

    /// Returns the length of the bone.
    pub fn length(&self) -> f32 {
        self.length
    }

    /// Sets the length of the bone.
    pub fn set_length(&mut self, length: f32) {
        self.length = length;
    }

    /// Returns the angle of the bone in radians.
    pub fn angle(&self) -> f32 {
        self.angle
    }

    /// Sets the angle of the bone in radians.
    pub fn set_angle(&mut self, angle: f32) {
        self.angle = angle;
    }

    /// Renders the bone using macroquad drawing functions.
    pub fn render(&self) {
        let end = self.end();

        // Draw bone as a line
        draw_line(self.start.x, self.start.y, end.x, end.y, 4.0, WHITE);

        // Draw joint markers
        draw_circle(self.start.x, self.start.y, 2.0, YELLOW);
        draw_circle(end.x, end.y, 2.0, GREEN);
    }
}
