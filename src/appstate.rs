use glutin::surface::{Surface, WindowSurface};
use winit::window::Window;

pub struct AppState {
    pub gl_surface: Surface<WindowSurface>,
    pub window: Window,
}
