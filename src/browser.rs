use std::collections::HashMap;

use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowId,
};

use wry::WebView;

use crate::window::create_window;

pub fn run() {
    let event_loop = EventLoop::new();

    // WindowIdで管理（これが正解）
    let mut views: HashMap<WindowId, (tao::window::Window, WebView)> = HashMap::new();

    let (window, webview) = create_window(&event_loop, "https://www.google.com");
    let id = window.id();
    views.insert(id, (window, webview));

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
                ..
            } => {
                views.remove(&window_id);

                if views.is_empty() {
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => {}
        }
    });
}
