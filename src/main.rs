mod bone;
mod skeleton;

use macroquad::prelude::*;
use skeleton::Skeleton;

#[macroquad::main("RustIK")]
async fn main() {
    let center = vec2(screen_width() / 2.0, screen_height() / 2.0);

    // Create skeleton
    let mut skeleton = Skeleton::new(Some(center));
    skeleton.add_bone(100.0, 0.0);
    skeleton.add_bone(100.0, 0.0);

    loop {
        // Get mouse pos as vector
        let target = Vec2::new(mouse_position().0, mouse_position().1);

        // Move skeleton to mouse pos
        if is_key_pressed(KeyCode::C) {
            skeleton.set_origin(target);
        }

        // Update skeleton
        skeleton.update(target);

        // Render
        clear_background(BLACK);
        skeleton.render();

        next_frame().await;
    }
}
