use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowId,
};

use wry::WebView;

use crate::window::create_window;

// IPCイベント共有キュー
type IpcQueue = Arc<Mutex<Vec<String>>>;

pub fn run() {
    let event_loop = EventLoop::new();

    let ipc_queue: IpcQueue = Arc::new(Mutex::new(Vec::new()));

    let mut views: HashMap<WindowId, (tao::window::Window, WebView)> = HashMap::new();

    // 初期タブ
    let (window, webview) = create_window(&event_loop, ipc_queue.clone());
    views.insert(window.id(), (window, webview));

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // IPC処理
        {
            let mut queue = ipc_queue.lock().unwrap();

            while let Some(msg) = queue.pop() {
                if msg == "new_tab" {
                    let (w, v) = create_window(&event_loop, ipc_queue.clone());
                    views.insert(w.id(), (w, v));
                }
            }
        }

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
