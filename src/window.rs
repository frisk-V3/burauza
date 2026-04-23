use std::sync::{Arc, Mutex};

use tao::event_loop::EventLoopWindowTarget;
use tao::window::{Window, WindowBuilder};
use tao::window::WindowId;
use wry::webview::WebView;
use wry::WebViewBuilder;

type IpcQueue = Arc<Mutex<Vec<(WindowId, String)>>>;

pub fn create_window(
    event_loop: &EventLoopWindowTarget<()>,
    ipc_queue: IpcQueue,
) -> (Window, WebView) {
    // Window を作る（EventLoopWindowTarget を渡す）
    let window = WindowBuilder::new()
        .with_title("Frisk Browser")
        .build(event_loop)
        .expect("Failed to create window");

    // HTML を埋め込み
    let html = include_str!("../assets/index.html");

    // WebView を作る。with_ipc_handler でウィンドウごとのメッセージをキューに入れる
    let webview = WebViewBuilder::new(&window)
        .with_html(html)
        .with_ipc_handler({
            let ipc_queue = ipc_queue.clone();
            move |window, msg| {
                let body = msg.to_string();
                let id = window.id();
                let mut queue = ipc_queue.lock().unwrap();
                queue.push((id, body));
            }
        })
        .build()
        .expect("Failed to build WebView");

    (window, webview)
}
