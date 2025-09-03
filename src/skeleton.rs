use crate::joint::Joint;

pub struct Skeleton<'a> {
    joints: Vec<Joint<'a>>,
}

impl<'a> Skeleton<'a> {
    pub fn new() -> Self {
        Self { joints: Vec::new() }
    }

    pub fn add_joint(&mut self, joint: Joint<'a>) {
        self.joints.push(joint);
    }

    pub fn render(&self) {
        for joint in self.joints.iter().enumerate() {
            joint.1.render();
        }
    }
}
