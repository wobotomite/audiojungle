use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    error::EventLoopError,
    event::{ElementState, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

use crate::{key_piano::KeyPiano, synth::Synth};

struct App {
    window: Option<Window>,
    piano: KeyPiano,
    synth: Synth,
}

pub fn run(synth: Synth) -> Result<(), EventLoopError> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop.run_app(&mut App {
        window: None,
        piano: KeyPiano::default(),
        synth,
    })
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let attributes = Window::default_attributes()
                .with_title("Audiojungle | A-K: piano | Z/X: octave | Space: bend | Esc: quit")
                .with_inner_size(LogicalSize::new(720.0, 180.0));
            self.window = Some(
                event_loop
                    .create_window(attributes)
                    .expect("could not create window"),
            );
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if self.window.as_ref().map(Window::id) != Some(window_id) {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                self.piano.release_all(&self.synth);
                event_loop.exit();
            }
            WindowEvent::Focused(false) => self.piano.release_all(&self.synth),
            WindowEvent::KeyboardInput {
                event,
                is_synthetic: false,
                ..
            } => {
                let PhysicalKey::Code(key) = event.physical_key else {
                    return;
                };
                if key == KeyCode::Escape && event.state == ElementState::Pressed {
                    self.piano.release_all(&self.synth);
                    event_loop.exit();
                } else {
                    self.piano
                        .handle_key(key, event.state, event.repeat, &self.synth);
                }
            }
            _ => {}
        }
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        self.piano.release_all(&self.synth);
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        self.piano.release_all(&self.synth);
    }
}
