use crate::env::*;

pub struct Window {}

impl Window {
    /// NEW WINDOW
    ///
    /// create a new window
    pub fn new() -> (sdl2::EventPump, sdl2::video::Window, sdl2::VideoSubsystem) {
        Window::new_custom(NAME, WIDTH, HEIGHT)
    }

    /// NEW WINDOW
    ///
    /// create a new custom window with name and size
    pub fn new_custom(
        name: &str,
        width: u32,
        height: u32,
    ) -> (sdl2::EventPump, sdl2::video::Window, sdl2::VideoSubsystem) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let gl_attr = video_subsystem.gl_attr();

        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 1);
        let window = video_subsystem
            .window(name, width, height)
            .opengl()
            .resizable()
            .build()
            .unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
        (event_pump, window, video_subsystem)
    }
}
