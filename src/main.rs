mod vulkan;
use vulkan::app::App;
use winit::{
    dpi,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub fn main() {
    #[cfg(debug_assertions)]
    println!("Debugging enabled");

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("scop")
        .with_inner_size(dpi::LogicalSize::new(400, 400)) // https://docs.rs/winit/latest/winit/dpi/index.html
        .build(&event_loop)
        .unwrap();

    let mut app = unsafe { App::init(&window).unwrap() };
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::MainEventsCleared => unsafe { app.main_loop(&window) }.unwrap(),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                *control_flow = ControlFlow::Exit;
                unsafe {
                    app.cleanup();
                }
            }
            _ => (),
        }
    });
}
