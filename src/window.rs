use tao::{
    event_loop::EventLoop,
    window::WindowBuilder,
};

use wry::WebViewBuilder;

// Windowじゃなくて WebView を返すのが正解
pub fn create_window(event_loop: &EventLoop<()>, url: &str) -> wry::WebView {
    let window = WindowBuilder::new()
        .with_title("Frisk Browser")
        .build(event_loop)
        .expect("Failed to create window");

    let webview = WebViewBuilder::new(window)
        .expect("WebView init failed")
        .with_url(url)
        .expect("Invalid URL")
        .build()
        .expect("Failed to build WebView");

    webview
}
