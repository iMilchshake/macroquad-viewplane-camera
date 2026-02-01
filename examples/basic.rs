use macroquad::prelude::*;
use macroquad_viewplane_camera::ViewplaneCamera;

const MARGIN: i32 = 100;

#[macroquad::main("basic")]
async fn main() {
    // create camera with plane dimensions [100, 100]
    let mut vp_cam = ViewplaneCamera::new(100.0, 100.0);

    loop {
        clear_background(WHITE);

        // set viewport
        vp_cam.set_viewport(
            MARGIN,
            MARGIN,
            screen_width() as i32 - 2 * MARGIN,
            screen_height() as i32 - 2 * MARGIN,
        );

        // handle scroll wheel zoom + mouse drag panning
        vp_cam.handle_inputs();

        // apply camera (sets macroquad camera internally)
        vp_cam.apply();

        // draw in plane coordinates
        draw_rectangle(0.0, 0.0, 100.0, 100.0, GRAY);

        let plane_mouse_pos = vp_cam.mouse_plane_pos().trunc();
        let mouse_in_plane = vp_cam.mouse_in_plane_view();

        draw_circle_lines(50.0, 50.0, 45.0, 5.0, DARKGRAY);
        if mouse_in_plane {
            draw_rectangle_lines(plane_mouse_pos.x, plane_mouse_pos.y, 1.0, 1.0, 0.1, BLACK);
            draw_line(
                0.0,
                0.0,
                plane_mouse_pos.x + 0.5,
                plane_mouse_pos.y + 0.5,
                0.05,
                BLACK,
            );
        }

        // draw boundary around plane, change color if mouse is inside plane
        let border_color = if mouse_in_plane { LIME } else { RED };
        draw_rectangle_lines(0., 0., 100., 100., 1.0, border_color);

        // restore default camera for rendering in global screen space (e.g. UI)
        vp_cam.reset_camera();

        // draw viewport boundaries in global space
        draw_rectangle_lines(
            MARGIN as f32,
            MARGIN as f32,
            screen_width() - 2. * MARGIN as f32,
            screen_height() - 2. * MARGIN as f32,
            2.0,
            BLACK,
        );

        // draw global and plane mouse position
        let mouse_pos_str = format!(
            "mouse position: global={:?} | plane={:?}",
            Vec2::from(mouse_position()),
            vp_cam.mouse_plane_pos().trunc()
        );
        draw_multiline_text(&mouse_pos_str, 16.0, 32.0, 32., None, BLACK);

        next_frame().await;
    }
}
