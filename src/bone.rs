use macroquad::prelude::*;

pub struct Bone {
    pub start: Vec2,
    pub length: f32,
    pub angle: f32,
}

impl Bone {
    pub fn new(start: Vec2, length: f32, angle: f32) -> Self {
        Self {
            start,
            length,
            angle,
        }
    }

    pub fn end(&self) -> Vec2 {
        self.start
            + vec2(
                self.length * self.angle.cos(),
                self.length * self.angle.sin(),
            )
    }

    pub fn render(&self) {
        let end = self.end();
        draw_line(self.start.x, self.start.y, end.x, end.y, 4.0, WHITE);

        // Draw circles at joints
        draw_circle(self.start.x, self.start.y, 2.0, YELLOW);
        draw_circle(end.x, end.y, 2.0, GREEN);
    }
}
