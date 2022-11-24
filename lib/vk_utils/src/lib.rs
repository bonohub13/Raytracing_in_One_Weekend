pub struct AppBase {
    entry: ash::Entry,
    event_loop: winit::event_loop::EventLoop<()>,
}

impl AppBase {
    pub fn new() -> Self {
        Self {
            entry: ash::Entry::linked(),
            event_loop: winit::event_loop::EventLoop::new(),
        }
    }

    pub fn entry(&self) -> &ash::Entry {
        &self.entry
    }

    pub fn run(&mut self) {
        use winit::platform::run_return::EventLoopExtRunReturn;

        self.event_loop.run_return(|event, _, control_flow| {
            use winit::event::{Event, WindowEvent};

            control_flow.set_poll();

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => control_flow.set_exit(),
                    _ => (),
                },
                Event::MainEventsCleared => {}
                _ => (),
            }
        });
    }
}
