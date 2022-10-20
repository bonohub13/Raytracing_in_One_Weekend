pub struct Window {
    config: super::WindowConfig,
    window: winit::window::Window,
}

impl Window {
    pub fn new(
        event_loop: &winit::event_loop::EventLoop<()>,
        config: &super::WindowConfig,
    ) -> Result<Self, String> {
        use winit::{
            dpi::LogicalSize,
            window::{Fullscreen, WindowBuilder},
        };

        let fullscreen: Option<Fullscreen> = if config.fullscreen {
            Some(Fullscreen::Borderless(None))
        } else {
            None
        };

        let result = WindowBuilder::new()
            .with_title(config.title.clone())
            .with_inner_size(LogicalSize::new(config.width, config.height))
            .with_resizable(config.resizable)
            .with_fullscreen(fullscreen)
            .build(event_loop);

        match result {
            Ok(window) => Ok(Self {
                config: config.clone(),
                window,
            }),
            Err(_) => Err(String::from("Failed to create window!")),
        }
    }

    pub fn framebuffer_size(&self) -> ash::vk::Extent2D {
        use ash::vk;

        let size = self.window.inner_size();

        vk::Extent2D {
            width: size.width,
            height: size.height,
        }
    }

    pub fn window_size(&self) -> ash::vk::Extent2D {
        use ash::vk;

        let size = self.window.outer_size();

        vk::Extent2D {
            width: size.width,
            height: size.height,
        }
    }

    pub fn close(control_flow: &mut winit::event_loop::ControlFlow) {
        use winit::event_loop::ControlFlow;

        *control_flow = ControlFlow::Exit;
    }

    pub fn is_minimized(&self) -> bool {
        let size = self.framebuffer_size();

        size.width == 0 && size.height == 0
    }

    pub fn run(event_loop: winit::event_loop::EventLoop<()>) {
        use winit::{
            event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
            event_loop::ControlFlow,
        };

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    WindowEvent::KeyboardInput { input, .. } => match input {
                        KeyboardInput {
                            virtual_keycode,
                            state,
                            ..
                        } => match (virtual_keycode, state) {
                            (Some(VirtualKeyCode::Escape), ElementState::Pressed) => {
                                *control_flow = ControlFlow::Exit;
                            }
                            _ => {}
                        },
                    },
                    _ => {}
                },
                Event::MainEventsCleared => {}
                _ => (),
            }
        });
    }
}
