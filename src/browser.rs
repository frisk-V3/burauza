use std::collections::HashMap;

use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use crate::window::create_window;

pub fn run() {
    let event_loop = EventLoop::new();

    // WindowじゃなくてWebView管理する
    let mut views: HashMap<u32, wry::WebView> = HashMap::new();
    let mut counter: u32 = 0;

    let view = create_window(&event_loop, "https://www.google.com");
    views.insert(counter, view);
    counter += 1;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
                ..
            } => {
                views.retain(|_, v| v.window().id() != window_id);

                if views.is_empty() {
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => {}
        }
    });
}
