use tao::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use wry::{WebView, WebViewBuilder};

pub fn create_window(event_loop: &EventLoop<()>) -> (Window, WebView) {
    let window = WindowBuilder::new()
        .with_title("Frisk Browser")
        .build(event_loop)
        .unwrap();

    let html = include_str!("../assets/index.html");

    let webview = WebViewBuilder::new(&window)
        .with_html(html)
        .unwrap()
        .with_ipc_handler(|window, msg| {
            let v: serde_json::Value = serde_json::from_str(&msg).unwrap();

            match v["cmd"].as_str().unwrap() {
                "new_tab" => {
                    // 新しいウィンドウ生成（タブ）
                    let _ = WindowBuilder::new()
                        .with_title("New Tab")
                        .build(window.event_loop())
                        .unwrap();
                }

                "navigate" => {
                    if let Some(url) = v["url"].as_str() {
                        window.set_title(url); // 仮
                    }
                }

                _ => {}
            }
        })
        .build()
        .unwrap();

    (window, webview)
}
