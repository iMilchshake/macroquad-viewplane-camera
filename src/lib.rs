use macroquad::camera::Camera2D;
use macroquad::color::colors::*;
use macroquad::input::{
    KeyCode, MouseButton, is_key_down, is_key_pressed, is_mouse_button_down,
    is_mouse_button_pressed, mouse_delta_position, mouse_position, mouse_wheel,
};
use macroquad::math::{Rect, Vec2};
use macroquad::shapes::{draw_circle, draw_line, draw_rectangle_lines};
use macroquad::window::{screen_height, screen_width};

/// Input configuration for ViewplaneCamera
struct InputConfig {
    zoom_factor: Option<f32>,
    pan_mouse_button: Option<MouseButton>,
    pan_wasd_step: Option<f32>,
    pan_arrow_step: Option<f32>,
    reset_key: Option<KeyCode>,
}

impl Default for InputConfig {
    fn default() -> InputConfig {
        InputConfig {
            zoom_factor: Some(0.9),
            pan_mouse_button: Some(MouseButton::Left),
            pan_wasd_step: None,
            pan_arrow_step: None,
            reset_key: Some(KeyCode::R),
        }
    }
}

pub struct ViewplaneCamera {
    offset: Vec2,
    zoom: f32,
    plane_size: Vec2,
    viewport: Option<Vec2>,
    viewport_ratio: Option<Vec2>,
    viewport_x_offset: Option<f32>,
    viewport_y_offset: Option<f32>,
    cam: Option<Camera2D>,
    config: InputConfig,
}

impl ViewplaneCamera {
    pub fn new(plane_width: f32, plane_height: f32) -> ViewplaneCamera {
        ViewplaneCamera {
            offset: Vec2::ZERO,
            zoom: 1.0,
            plane_size: Vec2::new(plane_width, plane_height),
            viewport: None,
            viewport_ratio: None,
            viewport_x_offset: None,
            viewport_y_offset: None,
            cam: None,
            config: InputConfig::default(),
        }
    }

    // ---- builder methods ----

    pub fn with_zoom_factor(mut self, factor: f32) -> Self {
        self.config.zoom_factor = Some(factor);
        self
    }

    pub fn without_zoom(mut self) -> Self {
        self.config.zoom_factor = None;
        self
    }

    pub fn with_mouse_pan(mut self, button: MouseButton) -> Self {
        self.config.pan_mouse_button = Some(button);
        self
    }

    pub fn without_mouse_pan(mut self) -> Self {
        self.config.pan_mouse_button = None;
        self
    }

    pub fn with_wasd_pan(mut self, step: f32) -> Self {
        self.config.pan_wasd_step = Some(step);
        self
    }

    pub fn with_arrow_key_pan(mut self, step: f32) -> Self {
        self.config.pan_arrow_step = Some(step);
        self
    }

    pub fn with_reset_key(mut self, key: KeyCode) -> Self {
        self.config.reset_key = Some(key);
        self
    }

    // ---- viewport ----

    pub fn set_viewport(&mut self, x: i32, y: i32, width: i32, height: i32) {
        let viewport = Vec2::new(width as f32, height as f32);
        self.viewport_ratio = Some(Vec2::new(
            viewport.x / screen_width(),
            viewport.y / screen_height(),
        ));
        self.viewport = Some(viewport);
        self.viewport_x_offset = Some(x as f32);
        self.viewport_y_offset = Some(y as f32);
    }

    // ---- transformations ----

    /// reset camera transformations
    pub fn reset(&mut self) {
        self.offset = Vec2::ZERO;
        self.zoom = 1.0;
    }

    /// zooms in or out by configured zoom factor
    pub fn zoom(&mut self, zoom_in: bool) {
        let zoom_factor = match self.config.zoom_factor {
            Some(f) => f,
            None => return,
        };
        match zoom_in {
            true => self.zoom /= zoom_factor,
            false => self.zoom *= zoom_factor,
        }
    }

    /// expects a "local" shift in [-1, +1] range wrt. to the full window size.
    /// if a viewport is used it will be scaled accordingly.
    pub fn shift(&mut self, local_shift: Vec2) {
        let viewport_ratio = self.viewport_ratio.expect("viewport not defined");
        let local_shift = local_shift / viewport_ratio;
        self.offset += local_shift / self.zoom;
    }

    // ---- camera application ----

    pub fn apply(&mut self) {
        let viewport = self.viewport.expect("viewport size not defined!");
        let plane_size = self.plane_size;

        // Calculate aspect ratio
        let viewport_ratio = viewport.x / viewport.y;
        let plane_ratio = plane_size.x / plane_size.y;
        let (cam_width, cam_height) = if viewport_ratio > plane_ratio {
            (plane_size.x * viewport_ratio / plane_ratio, plane_size.y)
        } else {
            (plane_size.x, plane_size.y * plane_ratio / viewport_ratio)
        };

        // set camera rect
        let mut cam = Camera2D::from_display_rect(Rect::new(0.0, 0.0, cam_width, cam_height));

        // apply user transformations
        cam.target = Vec2::new(
            (self.offset.x / cam.zoom.x) + (cam_width / 2.),
            (-self.offset.y / cam.zoom.y) + (cam_height / 2.),
        );
        cam.zoom *= self.zoom;
        cam.zoom.y *= -1.0; // Flip Y axis for macroquad 0.4 camera consistency

        // Set viewport position accounting for offsets (e.g. sidebar, menu bar)
        // Convert y from top-offset to OpenGL bottom-offset
        let viewport_x_offset = self.viewport_x_offset.unwrap();
        let viewport_y_offset = self.viewport_y_offset.unwrap();
        let opengl_y = screen_height() - viewport_y_offset - viewport.y;
        cam.viewport = Some((
            viewport_x_offset as i32,
            opengl_y as i32,
            viewport.x as i32,
            viewport.y as i32,
        ));

        macroquad::camera::set_camera(&cam);
        self.cam = Some(cam);
    }

    pub fn reset_camera(&self) {
        macroquad::camera::set_default_camera();
    }

    pub fn get_camera(&self) -> &Camera2D {
        self.cam.as_ref().unwrap()
    }

    // ---- coordinate conversion ----

    pub fn mouse_plane_pos(&self) -> Vec2 {
        let cam = self.cam.as_ref().expect("macroquad cam not defined");
        let (mouse_x, mouse_y) = mouse_position();

        // cam.viewport already accounts for the top offset, so we pass screen coords directly
        // screen_to_world() considers the Y flip via cam.zoom.y *= -1.0
        cam.screen_to_world(Vec2::new(mouse_x, mouse_y))
    }

    // ---- input handling ----

    fn mouse_in_viewport(&self) -> bool {
        let viewport = match &self.viewport {
            Some(v) => v,
            None => return false,
        };
        let vp_x = self.viewport_x_offset.unwrap_or(0.0);
        let vp_y = self.viewport_y_offset.unwrap_or(0.0);

        let (mouse_x, mouse_y) = mouse_position();

        // check if mouse is within viewport bounds (screen coordinates, y from top)
        mouse_x >= vp_x
            && mouse_x <= vp_x + viewport.x
            && mouse_y >= vp_y
            && mouse_y <= vp_y + viewport.y
    }

    pub fn handle_inputs(&mut self) {
        // handle reset key
        if let Some(key) = self.config.reset_key {
            if is_key_pressed(key) {
                self.reset();
            }
        }

        // handle zoom
        if self.config.zoom_factor.is_some() && mouse_wheel().1.abs() > 0.0 {
            self.zoom(mouse_wheel().1.is_sign_positive());
        }

        // handle mouse panning
        if let Some(button) = self.config.pan_mouse_button {
            let delta = mouse_delta_position();
            if is_mouse_button_down(button)
                && self.mouse_in_viewport()
                && !is_mouse_button_pressed(button)
            {
                self.shift(delta);
            }
        }

        // handle WASD panning
        if let Some(step) = self.config.pan_wasd_step {
            let mut pan = Vec2::ZERO;
            if is_key_down(KeyCode::W) {
                pan.y -= step;
            }
            if is_key_down(KeyCode::S) {
                pan.y += step;
            }
            if is_key_down(KeyCode::A) {
                pan.x += step;
            }
            if is_key_down(KeyCode::D) {
                pan.x -= step;
            }
            if pan != Vec2::ZERO {
                self.shift(pan);
            }
        }

        // handle arrow key panning
        if let Some(step) = self.config.pan_arrow_step {
            let mut pan = Vec2::ZERO;
            if is_key_down(KeyCode::Up) {
                pan.y -= step;
            }
            if is_key_down(KeyCode::Down) {
                pan.y += step;
            }
            if is_key_down(KeyCode::Left) {
                pan.x += step;
            }
            if is_key_down(KeyCode::Right) {
                pan.x -= step;
            }
            if pan != Vec2::ZERO {
                self.shift(pan);
            }
        }
    }

    // ---- debug draws ----

    pub fn draw_debug(&self) {
        let plane_size = self.plane_size;
        let cam = self.cam.as_ref().expect("macroquad cam not defined");
        let mouse_pos = self.mouse_plane_pos();

        draw_circle(mouse_pos.x, mouse_pos.y, 1.0, BLUE);

        draw_line(0.0, 0.0, plane_size.x, plane_size.y, 2., BLUE);
        draw_rectangle_lines(0.0, 0.0, plane_size.x, plane_size.y, 2.0, RED);
        draw_circle(plane_size.x / 2., plane_size.y / 2., 2.0, LIME);
        draw_circle(cam.target.x, cam.target.y, 2.0, DARKBLUE);
    }
}
