use wry::{
    application::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
    webview::WebViewBuilder,
};

pub fn run() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Frisk Browser")
        .build(&event_loop)
        .unwrap();

    let html = include_str!("../assets/index.html");

    let _webview = WebViewBuilder::new(window)
        .unwrap()
        .with_html(html)
        .unwrap()
        .with_ipc_handler(|_window, msg| {
            println!("JSから: {}", msg);

            // TODO: タブ管理・履歴保存などここで処理
        })
        .build()
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }
    });
}
