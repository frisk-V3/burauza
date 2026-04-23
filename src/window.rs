use tao::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use wry::{WebView, WebViewBuilder};

// Window + WebView をセットで返す
pub fn create_window(event_loop: &EventLoop<()>, url: &str) -> (Window, WebView) {
    let window = WindowBuilder::new()
        .with_title("Frisk Browser")
        .build(event_loop)
        .expect("Failed to create window");

    let webview = WebViewBuilder::new(&window) // ← 参照！
        .with_url(url)
        .expect("Invalid URL")
        .build()
        .expect("Failed to build WebView");

    (window, webview)
}
