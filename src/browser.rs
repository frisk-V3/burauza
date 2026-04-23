use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    window::WindowId,
};

use wry::WebView;

use crate::window::create_window;

// IpcQueue は (WindowId, String) のキュー
type IpcQueue = Arc<Mutex<Vec<(WindowId, String)>>>;

pub fn run() {
    let event_loop = EventLoop::new();

    let ipc_queue: IpcQueue = Arc::new(Mutex::new(Vec::new()));

    // WindowId -> (Window, WebView)
    let mut views: HashMap<WindowId, (tao::window::Window, WebView)> = HashMap::new();

    // 最初のタブ（EventLoop を &event_loop として渡す）
    let (window, webview) = create_window(&event_loop, ipc_queue.clone());
    views.insert(window.id(), (window, webview));

    // run に渡すクロージャは第2引数で EventLoopWindowTarget を受け取れる
    event_loop.run(move |event, event_loop_target, control_flow| {
        *control_flow = ControlFlow::Wait;

        // まず IPC キューを処理
        {
            let mut queue = ipc_queue.lock().unwrap();

            while let Some((sender_id, msg)) = queue.pop() {
                // new_tab の場合は新しいウィンドウを作る
                if msg == "new_tab" {
                    let (w, v) = create_window(event_loop_target, ipc_queue.clone());
                    views.insert(w.id(), (w, v));
                    continue;
                }

                // navigate:<url> の場合は該当ウィンドウの WebView をナビゲート
                if let Some(rest) = msg.strip_prefix("navigate:") {
                    if let Some((_win, webview)) = views.get_mut(&sender_id) {
                        // WebView::navigate を呼ぶ
                        let _ = webview.navigate(rest);
                    }
                    continue;
                }

                // その他のメッセージは今はログ出力
                println!("IPC from {:?}: {}", sender_id, msg);
            }
        }

        // ウィンドウイベント処理（CloseRequested で views から削除）
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
