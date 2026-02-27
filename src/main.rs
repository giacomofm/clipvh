use std::num::NonZeroU32;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

const BACKGROUND_COLOR: u32 = 0xff181818;
const WINDOW_WIDTH: u32 = 300;
const WINDOW_HEIGHT: u32 = 400;

#[derive(Default)]
struct App {
    window: Option<Arc<Window>>,
    surface: Option<softbuffer::Surface<Arc<Window>, Arc<Window>>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title("clipvh")
            .with_inner_size(LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
            .with_resizable(false);
        let window = match event_loop.create_window(window_attributes) {
            Ok(w) => Arc::new(w),
            Err(err) => {
                eprintln!("error creating window: {err}");
                event_loop.exit();
                return;
            }
        };
        let context = softbuffer::Context::new(window.clone()).unwrap();
        let surface = softbuffer::Surface::new(&context, window.clone()).unwrap();
        self.window = Some(window);
        self.surface = Some(surface);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        println!("{event:?}");
        match event {
            WindowEvent::CloseRequested => {
                println!("Close was requested; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.surface
                    .as_mut()
                    .unwrap()
                    .resize(
                        NonZeroU32::new(WINDOW_WIDTH).unwrap(),
                        NonZeroU32::new(WINDOW_HEIGHT).unwrap(),
                    )
                    .expect("Failed to resize the softbuffer surface");
                let mut buffer = self
                    .surface
                    .as_mut()
                    .unwrap()
                    .buffer_mut()
                    .expect("Failed to get the softbuffer buffer");
                buffer.fill(BACKGROUND_COLOR);
                buffer
                    .present()
                    .expect("Failed to present the softbuffer buffer");
                // self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
