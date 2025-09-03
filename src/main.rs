mod bone;
mod joint;
mod skeleton;

use bone::Bone;
use joint::Joint;
use macroquad::prelude::*;
use skeleton::Skeleton;

#[macroquad::main("RustIK")]
async fn main() {
    let center = vec2(screen_width() / 2.0, screen_height() / 2.0);

    // Create bones
    let bone1 = Bone::new(center, 100.0, 0.5);
    let bone2 = Bone::new(bone1.end(), 80.0, 1.0);

    // Link bones
    let joint = Joint::new(&bone1, &bone2);

    // Create skeleton
    let mut skeleton = Skeleton::new();
    skeleton.add_joint(joint);

    loop {
        clear_background(BLACK);

        // Render skeleton
        skeleton.render();

        next_frame().await
    }
}
