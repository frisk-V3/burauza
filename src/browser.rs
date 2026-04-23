use std::collections::HashMap;

use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use crate::window::create_window;

pub fn run() {
    let event_loop = EventLoop::new();

    let mut windows: HashMap<u32, tao::window::Window> = HashMap::new();
    let mut counter: u32 = 0;

    // 初期タブ（= ウィンドウ）
    let win = create_window(&event_loop, "https://www.google.com");
    windows.insert(counter, win);
    counter += 1;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
                ..
            } => {
                // 該当ウィンドウ削除
                windows.retain(|_, w| w.id() != window_id);

                // 全部閉じたら終了
                if windows.is_empty() {
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => {}
        }
    });
}
