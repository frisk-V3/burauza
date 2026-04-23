use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopProxy},
    window::WindowId,
};

use wry::WebView;

use crate::window::{create_window, UserEvent};

type IpcQueue = Arc<Mutex<Vec<String>>>;

pub fn run() {
    // ✅ ここが最終正解
    let event_loop = EventLoop::<UserEvent>::with_user_event();

    let proxy: EventLoopProxy<UserEvent> = event_loop.create_proxy();

    let ipc_queue: IpcQueue = Arc::new(Mutex::new(Vec::new()));

    let mut views: HashMap<WindowId, (tao::window::Window, WebView)> = HashMap::new();

    let (window, webview) = create_window(&event_loop, ipc_queue.clone(), proxy.clone());
    views.insert(window.id(), (window, webview));

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        {
            let mut queue = ipc_queue.lock().unwrap();
            while let Some(msg) = queue.pop() {
                if msg == "new_tab" {
                    proxy.send_event(UserEvent::NewTab).ok();
                }
            }
        }

        match event {
            Event::UserEvent(UserEvent::NewTab) => {
                let (w, v) = create_window(&event_loop, ipc_queue.clone(), proxy.clone());
                views.insert(w.id(), (w, v));
            }

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
