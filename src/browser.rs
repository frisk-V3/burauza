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

    let mut views: HashMap<WindowId, (tao::window::Window, WebView)> = HashMap::new();

    // 初期タブ（ホーム）
    let (window, webview) = create_window(&event_loop);
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
