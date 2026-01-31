use macroquad::prelude::*;
use macroquad_viewplane_camera::ViewplaneCamera;

#[macroquad::main("basic")]
async fn main() {
    // create camera with plane dimensions (the area you want to render)
    let mut vp_cam = ViewplaneCamera::new(100.0, 100.0).with_wasd_pan(0.1);

    loop {
        clear_background(BLACK);

        // set viewport position and size (leave 200px for sidebar on the right)
        let sidebar_width = 200;
        let viewport_width = screen_width() as i32 - sidebar_width;
        vp_cam.set_viewport(0, 0, viewport_width, screen_height() as i32);

        // handle scroll wheel zoom + mouse drag panning
        vp_cam.handle_inputs();

        // apply camera (sets macroquad camera internally)
        vp_cam.apply();

        // draw in plane coordinates (0,0) to (100,100)
        draw_rectangle(0.0, 0.0, 100.0, 100.0, DARKGRAY);
        draw_circle(50.0, 50.0, 10.0, RED);

        // restore default camera for UI rendering etc.
        vp_cam.reset_camera();

        // draw sidebar in screen coordinates
        draw_rectangle(
            viewport_width as f32,
            0.0,
            sidebar_width as f32,
            screen_height(),
            GRAY,
        );

        next_frame().await;
    }
}
