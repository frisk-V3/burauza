use tao::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use wry::{WebView, WebViewBuilder};

pub fn create_window(event_loop: &EventLoop<()>, url: &str) -> (Window, WebView) {
    let window = WindowBuilder::new()
        .with_title("Frisk Browser")
        .build(event_loop)
        .expect("Failed to create window");

    let webview = WebViewBuilder::new(&window)
        .with_url(url)   // ← ここはそのまま（Resultじゃない）
        .build()         // ← ここだけResult
        .expect("Failed to build WebView");

    (window, webview)
}
