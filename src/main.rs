mod raytracing_in_one_weekend;

use raytracing_in_one_weekend::{AppBase, RayTracingInOneWeekend};
use winit::event_loop::EventLoop;

fn main() {
    let event_loop = EventLoop::new();
    let window = AppBase::init_window(&event_loop).unwrap();

    let app = RayTracingInOneWeekend::new(&window);

    AppBase::main_loop(event_loop);
}
