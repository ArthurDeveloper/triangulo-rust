#![allow(unused_variables)]

use glium::{glutin, Display, Surface, Program, VertexBuffer, texture::pixel_buffer::PixelBuffer,
            index};
use glutin::{window::WindowBuilder, ContextBuilder, event_loop::{EventLoop, ControlFlow},
             event::{Event, WindowEvent, StartCause}};

pub mod file_reader;
pub mod vertex;

use file_reader::FileReader;
use vertex::Vertex;

fn main() {
    let event_loop = EventLoop::new();
    let context = ContextBuilder::new();
    let window = WindowBuilder::new()
                    .with_inner_size(glutin::dpi::LogicalSize::new(640.0, 480.0))
                    .with_title("Triangle");
    let display = Display::new(window, context, &event_loop).unwrap();

    let vertex_shader = FileReader::new("shaders/triangle_vert.glsl").file_content;
    let fragment_shader = FileReader::new("shaders/triangle_frag.glsl").file_content;

    let program = Program::from_source(&display, &vertex_shader, &fragment_shader, None).unwrap();

    let shape = vec![Vertex { position: [-0.5, -0.5]}, Vertex { position: [0.0, 0.5] }, 
                     Vertex { position: [ 0.5, -0.5]}];

    let vertex_buffer = VertexBuffer::new(&display, &shape).unwrap();

    let indices = index::NoIndices(index::PrimitiveType::TrianglesList);

    event_loop.run(move | event, _, control_flow | {
        let next_frame_time = std::time::Instant::now() 
                              + std::time::Duration::from_nanos(16_666_667);
        *control_flow = ControlFlow::WaitUntil(next_frame_time);
        
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            Event::NewEvents(cause) => match cause {
                StartCause::ResumeTimeReached { .. } => (),
                StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms, 
                    &Default::default()).unwrap();
        target.finish().unwrap();
    });
}
