# macroquad-viewplane-camera

Dynamic and easy rendering of a 2D plane using Macroquad's `Camera2D`.
This crate removes the usual hassle of setting up a robust 2D camera system in Macroquad and lets you focus on building your actual project.
Draw calls can be performed in a local coordinate space (e.g. a game level) and projected onto the window, handling panning, zooming, window resizing and viewport constraints.
The view can be constrained to a dynamically resizable viewport inside the window, making it easy to integrate with UI elements such as a sidebar.
This crate is ideal for building simulations or editor-style applications.

## Installation

_TODO_

## Quick Start

_TODO_

## Configuration

The viewplane camera uses a builder pattern for configuration:

```rust
let mut vp_cam = ViewplaneCamera::new(100.0, 100.0)
    // zoom
    .with_zoom_factor(0.85)              // zoom speed (default: 0.9)

    // mouse panning
    .with_mouse_pan(MouseButton::Middle) // change pan button (default: Left)

    // keyboard panning (disabled by default)
    .with_wasd_pan(0.05)                 // enable WASD panning with step size
    .with_arrow_key_pan(0.05)            // enable arrow key panning with step size

    // reset pan/zoom to default
    .with_reset_key(KeyCode::Q);         // press Q to reset pan/zoom (default: R)
```

Disable mouse panning / zooming:

```rust
let mut vp_cam = ViewplaneCamera::new(100.0, 100.0)
    .without_mouse_pan()
    .without_zoom();
```

## Manual Input Handling

The built-in input system can be completely disabled by simply not calling `handle_inputs()`.
You can use the low-level methods instead for full control:

```rust
// zoom in/out by configured factor
vp_cam.zoom(true);   // zoom in
vp_cam.zoom(false);  // zoom out

// pan by local shift (in normalized screen coordinates)
vp_cam.shift(Vec2::new(dx, dy));

// reset pan and zoom to initial state
vp_cam.reset();
```

## Coordinate Conversion

Get mouse position in plane coordinates:

```rust
let plane_pos = vp_cam.mouse_plane_pos();
```

## Extracting the Macroquad Camera

When you need access to the underlying `Camera2D`:

```rust
let mq_cam: &Camera2D = vp_cam.get_camera();
```

## Viewport Integration

The viewport can be resized dynamically, useful for editor layouts with sidebars:

```rust
// leave 200px on the right for a sidebar
let viewport_width = screen_width() as i32 - 200;
vp_cam.set_viewport(0, 0, viewport_width, screen_height() as i32);
```

For egui integration, convert the available rect:

```rust
// after egui layout
let rect = egui_ctx.available_rect();
vp_cam.set_viewport(
    rect.min.x as i32,
    rect.min.y as i32,
    (rect.max.x - rect.min.x) as i32,
    (rect.max.y - rect.min.y) as i32,
);
```
