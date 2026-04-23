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
        .with_initialization_script(include_str!("../assets/script.js"))
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
