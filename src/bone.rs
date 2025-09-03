use macroquad::prelude::*;

pub struct Bone {
    start: Vec2,
    length: f32,
    angle: f32,
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
            + Vec2::new(
                self.length * self.angle.cos(),
                self.length * self.angle.sin(),
            )
    }

    pub fn render(&self) {
        let end = self.end();
        draw_line(self.start.x, self.start.y, end.x, end.y, 4.0, WHITE);
    }
}
