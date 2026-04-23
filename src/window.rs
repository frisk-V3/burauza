use wry::{
    application::{
        event_loop::EventLoop,
        window::{WindowBuilder, Window},
    },
    webview::WebViewBuilder,
};

pub fn create_window(event_loop: &EventLoop<()>, url: &str) -> Window {
    let window = WindowBuilder::new()
        .with_title("Frisk Browser")
        .build(event_loop)
        .unwrap();

    WebViewBuilder::new(window.clone())
        .unwrap()
        .with_url(url)
        .unwrap()
        .with_ipc_handler(|window, msg| {
            if msg == "new_tab" {
                // 新しいウィンドウを開く（擬似タブ）
                let _ = WindowBuilder::new()
                    .with_title("New Tab")
                    .build(window.event_loop())
                    .unwrap();
            }
        })
        .build()
        .unwrap();

    window
}
