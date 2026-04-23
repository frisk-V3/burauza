use std::collections::HashMap;
use wry::application::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use crate::window::create_window;

pub fn run() {
    let event_loop = EventLoop::new();

    let mut windows = HashMap::new();
    let mut counter = 0;

    // 初期タブ
    let win = create_window(&event_loop, "https://www.google.com");
    windows.insert(counter, win);
    counter += 1;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
                ..
            } => {
                windows.retain(|_, w| w.id() != window_id);

                if windows.is_empty() {
                    *control_flow = ControlFlow::Exit;
                }
            }

            _ => {}
        }
    });
}
