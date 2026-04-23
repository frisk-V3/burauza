use tao::{
    event_loop::{EventLoop, EventLoopProxy},
    window::{Window, WindowBuilder},
};

use wry::{WebView, WebViewBuilder};

use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub enum UserEvent {
    NewTab,
}

type IpcQueue = Arc<Mutex<Vec<String>>>;

pub fn create_window(
    event_loop: &EventLoop<UserEvent>,
    ipc_queue: IpcQueue,
    proxy: EventLoopProxy<UserEvent>,
) -> (Window, WebView) {
    let window = WindowBuilder::new()
        .with_title("Frisk Browser")
        .build(event_loop)
        .expect("Failed to create window");

    let html = include_str!("../assets/index.html");

    let webview = WebViewBuilder::new(&window)
        .with_html(html)
        .with_ipc_handler(move |req| {
            let body = req.body().to_string();

            if body == "new_tab" {
                proxy.send_event(UserEvent::NewTab).ok();
            } else {
                let mut queue = ipc_queue.lock().unwrap();
                queue.push(body);
            }
        })
        .build()
        .expect("Failed to build WebView");

    (window, webview)
}
