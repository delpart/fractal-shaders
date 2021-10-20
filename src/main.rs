#[macro_use]
extern crate glium;

#[allow(unused_imports)]
use glium::{glutin, Surface};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

fn main() {

    let events_loop = glium::glutin::event_loop::EventLoop::new();

    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(1024.0, 768.0))
        .with_title("Fractal Shader");

    let cb = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();

    let program = glium::Program::from_source(
        &display,
        include_str!("fractal.vert"),
        include_str!("fractal.frag"),
        None).unwrap();

    let vertices = [
        Vertex{ position: [-1.0,  1.0] },
        Vertex{ position: [ 1.0,  1.0] },
        Vertex{ position: [-1.0, -1.0] },

        Vertex { position: [-1.0, -1.0] },
        Vertex { position: [ 1.0,  1.0] },
        Vertex { position: [ 1.0, -1.0] },
    ];

    let vertex_buffer = glium::VertexBuffer::new(&display, &vertices).unwrap();

    let index_buffer = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);


    events_loop.run(move |event, _, control_flow|{

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glium::glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match event {
            glium::glutin::event::Event::WindowEvent { event, .. } => match event {
                glium::glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glium::glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            glium::glutin::event::Event::NewEvents(cause) => match cause {
                glium::glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glium::glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let (x, y) = display.get_framebuffer_dimensions();

        let mut target = display.draw();
        
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        target.draw(&vertex_buffer,
                    &index_buffer,
                    &program,
                    &uniform! {size: [x as f32, y as f32]},
      &Default::default()
            ).unwrap();
        target.finish().unwrap();
    });
}
