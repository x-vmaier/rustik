use macroquad::prelude::*;

#[macroquad::main("RustIK")]
async fn main() {
    loop {
        clear_background(BLACK);
        draw_line(screen_width() / 2.0, screen_height() / 2.0, mouse_position().0, mouse_position().1, 10.0, WHITE);
        next_frame().await
    }
}
