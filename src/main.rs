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
    
    let (mut cx, mut cy) = (0.0, 0.0);
    let mut mouse_position: Option<(i32, i32)> = None;
    let mut zoom_step = 1.0;
    let mut zoom = 1.0;
    let mut order = 5;
    let mut max_iter = 60;
    
    let duration = std::time::Instant::now();

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
                glium::glutin::event::WindowEvent::MouseWheel{delta, ..} => match delta {
                        glium::glutin::event::MouseScrollDelta::LineDelta(_x, y) => {
                            zoom_step += y;
                            return;
                        }
                        glium::glutin::event::MouseScrollDelta::PixelDelta(x) => {
                            zoom_step += x.y as f32;
                            return;
                        }
                },
                glutin::event::WindowEvent::CursorMoved { position, .. } => {
                    mouse_position = Some(position.cast::<i32>().into());
                    return;
                },
                glium::glutin::event::WindowEvent::MouseInput{state, button, ..} => match button{
                    glium::glutin::event::MouseButton::Left => {
                        if state == glium::glutin::event::ElementState::Pressed {
                            if let Some(position) = mouse_position{
                                cx += (position.0 as f32 - display.get_framebuffer_dimensions().0 as f32 / 2.)*zoom;
                                cy += (display.get_framebuffer_dimensions().1 as f32/2. - position.1 as f32)*zoom;
                            }
                        }
                    },
                    _ => return,
                },
                glium::glutin::event::WindowEvent::KeyboardInput{input, ..} => {
                    if let Some(key) = input.virtual_keycode{
                        match key {
                            glutin::event::VirtualKeyCode::Key1 => {
                                order = 1;
                                return;
                            },
                            glutin::event::VirtualKeyCode::Key2 => {
                                order = 2;
                                return;
                            },
                            glutin::event::VirtualKeyCode::Key3 => {
                                order = 3;
                                return;
                            },
                            glutin::event::VirtualKeyCode::Key4 => {
                                order = 4;
                                return;
                            },
                            glutin::event::VirtualKeyCode::Key5 => {
                                order = 5;
                                return;
                            },
                            glutin::event::VirtualKeyCode::Key6 => {
                                order = 6;
                                return;
                            },
                            glutin::event::VirtualKeyCode::Key7 => {
                                order = 7;
                                return;
                            },
                            glutin::event::VirtualKeyCode::Key8 => {
                                order = 8;
                                return;
                            },
                            glutin::event::VirtualKeyCode::Key9 => {
                                order = 9;
                                return;
                            },
                            glutin::event::VirtualKeyCode::Key0 => {
                                order = 10;
                                return;
                            },
                            glutin::event::VirtualKeyCode::F1 => {
                                max_iter = 1;
                                return;
                            },
                            glutin::event::VirtualKeyCode::F2 => {
                                max_iter = 10;
                                return;
                            },
                            glutin::event::VirtualKeyCode::F3 => {
                                max_iter = 30;
                                return;
                            },
                            glutin::event::VirtualKeyCode::F4 => {
                                max_iter = 60;
                                return;
                            },
                            glutin::event::VirtualKeyCode::F5 => {
                                max_iter = 120;
                                return;
                            },
                            _ => return,
                        }
                    }
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

        zoom = (zoom_step/5.).exp();

        let (x, y) = display.get_framebuffer_dimensions();

        let mut target = display.draw();
        
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        target.draw(&vertex_buffer,
                    &index_buffer,
                    &program,
                    &uniform! {size: [x as f32, y as f32], center: [cx, cy], zoom: zoom as f32, t: duration.elapsed().as_secs_f32(), max_iter: max_iter, order: order},
      &Default::default()
            ).unwrap();
        target.finish().unwrap();
    });
}
