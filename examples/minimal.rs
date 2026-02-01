use macroquad::prelude::*;
use macroquad_viewplane_camera::ViewplaneCamera;

#[macroquad::main("minimal")]
async fn main() {
    // create camera with plane dimensions [100, 100]
    let mut vp_cam = ViewplaneCamera::new(100.0, 100.0);

    loop {
        clear_background(WHITE);

        // set viewport with 200px sidebare
        let x_sidebar = screen_width() - 200.;
        vp_cam.set_viewport(0, 0, x_sidebar as i32, screen_height() as i32);

        // handle scroll wheel zoom + mouse drag panning
        vp_cam.handle_inputs();

        // apply camera (sets macroquad camera internally)
        vp_cam.apply();

        // draw in plane coordinates
        vp_cam.draw_debug();

        // restore default camera for rendering in global screen space (e.g. UI)
        vp_cam.reset_camera();

        // draw sidebar/viewport boundary in global space
        draw_line(x_sidebar, 0.0, x_sidebar, screen_height(), 2.0, BLACK);

        next_frame().await;
    }
}
