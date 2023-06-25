use glutin::config::GetGlConfig;
use glutin::context::NotCurrentGlContext;
use glutin::context::PossiblyCurrentGlContext;
use glutin::display::GetGlDisplay;
use glutin::display::GlDisplay;
use glutin_winit::GlWindow;
use raw_window_handle::HasRawWindowHandle;

fn main() {
    let event_loop = winit::event_loop::EventLoop::<()>::new();

    let (window, gl_config) = glutin_winit::DisplayBuilder::new()
        .with_window_builder(Some(winit::window::WindowBuilder::new()))
        .build(&event_loop, Default::default(), |configs| configs.last().unwrap())
        .unwrap();
    let window = window.unwrap();
    let raw_window_handle = window.raw_window_handle();
    let gl_display = gl_config.display();
    let context_attributes =
        glutin::context::ContextAttributesBuilder::new().build(Some(raw_window_handle));

    let gl_ctx = unsafe {
        gl_display
            .create_context(&gl_config, &context_attributes)
            .expect("Failed to create context")
    };

    let gl_ctx = gl_ctx.treat_as_possibly_current();

    let mut state = Some((window, None));

    event_loop.run(move |e, _, flow| match e {
        winit::event::Event::WindowEvent {
            event: winit::event::WindowEvent::CloseRequested,
            ..
        } => *flow = winit::event_loop::ControlFlow::Exit,
        winit::event::Event::Resumed => {
            eprintln!("Resumed!");
            let (window, _) = state.take().unwrap();
            let gl_config = gl_ctx.config();
            let attrs = window.build_surface_attributes(Default::default());

            // // If this one is never created everything is ok
            let unused_gl_surface =
                unsafe { gl_config.display().create_window_surface(&gl_config, &attrs).unwrap() };

            let gl_surface =
                unsafe { gl_config.display().create_window_surface(&gl_config, &attrs).unwrap() };

            state.replace((window, Some(gl_surface)));
        },
        _ => {},
    });
}
