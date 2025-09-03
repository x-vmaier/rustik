use crate::bone::Bone;
use macroquad::prelude::*;

pub struct Joint<'a> {
    parent: &'a Bone,
    child: &'a Bone,
}

impl<'a> Joint<'a> {
    pub fn new(parent: &'a Bone, child: &'a Bone) -> Self {
        Self { parent, child }
    }

    pub fn render(&self) {
        let pos = self.parent.end();
        draw_circle(pos.x, pos.y, 10.0, GREEN);
    }
}
