use tao::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use wry::{WebView, WebViewBuilder};

use std::sync::{Arc, Mutex};

type IpcQueue = Arc<Mutex<Vec<String>>>;

pub fn create_window(
    event_loop: &EventLoop<()>,
    ipc_queue: IpcQueue,
) -> (Window, WebView) {
    let window = WindowBuilder::new()
        .with_title("Frisk Browser")
        .build(event_loop)
        .expect("Failed to create window");

    let html = include_str!("../assets/index.html");

    let webview = WebViewBuilder::new(&window)
        .with_html(html)
        .with_ipc_handler(move |req| {
            let body = req.body();

            // そのままキューに積む（JSONでも文字列でもOK）
            let mut queue = ipc_queue.lock().unwrap();
            queue.push(body.to_string());
        })
        .build()
        .expect("Failed to build WebView");

    (window, webview)
}
