use macroquad::prelude::*;
use macroquad_viewplane_camera::ViewplaneCamera;

#[macroquad::main("basic")]
async fn main() {
    // create camera with plane dimensions (the area you want to render)
    let mut vp_cam = ViewplaneCamera::new(100.0, 100.0).with_wasd_pan(0.1);
    // .with_bot_anchor();

    loop {
        clear_background(WHITE);

        // set viewport
        vp_cam.set_viewport(
            50,
            50,
            screen_width() as i32 - 100,
            screen_height() as i32 - 100,
        );

        // handle scroll wheel zoom + mouse drag panning
        vp_cam.handle_inputs();

        // apply camera (sets macroquad camera internally)
        vp_cam.apply();

        // draw in plane coordinates (0,0) to (100,100)
        draw_rectangle(0.0, 0.0, 100.0, 100.0, GRAY);
        draw_circle_lines(50.0, 50.0, 10.0, 2.0, BLACK);
        draw_line(0.0, 0.0, 50.0, 50.0, 0.1, BLACK);

        let border_color = if vp_cam.mouse_in_plane_view() {
            LIME
        } else {
            RED
        };
        draw_rectangle_lines(0., 0., 100., 100., 1.0, border_color);
        // restore default camera for UI rendering etc.
        vp_cam.reset_camera();

        next_frame().await;
    }
}
