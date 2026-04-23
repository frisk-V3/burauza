use tao::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use wry::{WebView, WebViewBuilder};

use serde_json::Value;

pub fn create_window(event_loop: &EventLoop<()>) -> (Window, WebView) {
    let window = WindowBuilder::new()
        .with_title("Frisk Browser")
        .build(event_loop)
        .expect("Failed to create window");

    let html = include_str!("../assets/index.html");

    let webview = WebViewBuilder::new(&window)
        .with_html(html)
        .expect("Invalid HTML")
        .with_ipc_handler(move |_window, msg| {
            let v: Value = serde_json::from_str(&msg).unwrap();

            match v["cmd"].as_str().unwrap_or("") {
                "new_tab" => {
                    // ⚠️ ここでは直接作れない（event_loopが無い）
                    // → とりあえずログ確認
                    println!("New tab requested");
                }
                "navigate" => {
                    if let Some(url) = v["url"].as_str() {
                        println!("Navigate to: {}", url);
                    }
                }
                _ => {}
            }
        })
        .build()
        .expect("Failed to build WebView");

    (window, webview)
}
