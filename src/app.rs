use crate::base::{AppBase, Config};
use std::{cell::RefCell, rc::Rc};
use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop,
    window::WindowId,
};

#[derive(Debug, Default)]
pub struct App<'window> {
    base: Option<AppBase<'window>>,
    config: Option<Rc<RefCell<Config>>>,
}

impl App<'_> {
    pub fn set_config(&mut self, config: &Config) {
        self.config = Some(Rc::new(RefCell::new(Config { fps: config.fps })));
    }
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.base.is_none()
            && let Some(config) = self.config.as_ref()
        {
            match AppBase::new(event_loop, &config.borrow()) {
                Ok(base) => self.base = Some(base),
                Err(e) => panic!("{}", e),
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        if let Some(base) = self.base.as_mut() {
            match event {
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                WindowEvent::Resized(size) => {
                    base.resize(size);
                }
                WindowEvent::RedrawRequested => {
                    base.redraw_frame();
                }
                _ => (),
            }
        }
    }
}
