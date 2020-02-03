use winit::{WindowEvent, ElementState, VirtualKeyCode, KeyboardInput};

use rand::prelude::*;
#[macro_use]
extern crate vulkano;

mod vulkan_application;
use vulkan_application::VulkanApplication;

#[derive(Default, Copy, Clone, Debug)]
pub struct Vertex {
    pos: [f32; 2],
    color: [f32; 3],
}


impl Vertex {
    fn new(pos: [f32; 2], color: [f32; 3]) -> Self {
        Self { pos, color }
    }
}

vulkano::impl_vertex!(Vertex, pos, color);

pub fn vertices() -> Vec<Vertex> {

    [
        Vertex::new([-0.5, -0.5], [rand::thread_rng().gen(), 0.0, 0.0]),
        Vertex::new([0.5, -0.5], [rand::thread_rng().gen(), 1.0, 0.0]),
        Vertex::new([0.5, 0.5], [0.0, rand::thread_rng().gen(), 1.0]),
        Vertex::new([-0.5, 0.5], [1.0, 1.0, rand::thread_rng().gen()]),
        Vertex::new([rand::thread_rng().gen(), rand::thread_rng().gen()], [1.0, 1.0, 1.0])
    ].to_vec()
}

pub fn indices() -> Vec<u16> {
    [0, 1, 2, 2, 3, 0, 0, 5, 3].to_vec()
}

fn main() {
    let mut app = VulkanApplication::initialize();
    println!("main_loop");
    loop {
        println!("draw_frame" );
        app.draw_frame(&vertices(), &indices());
        let mut done = false;
        app.events_loop.run_forever(|event| {
            // println!("{:?}", event);

            match event {
                winit::Event::WindowEvent {
                    event: winit::WindowEvent::CloseRequested,
                    ..
                } => {  done = true; winit::ControlFlow::Break},
                winit::Event::WindowEvent {
                    event: WindowEvent::KeyboardInput { input, .. },
                    ..
                } => match input {
                    KeyboardInput {
                        virtual_keycode: Some(key),
                        state: ElementState::Pressed,
                        ..
                    } => match key {
                        VirtualKeyCode::M => {
                            winit::ControlFlow::Continue
                        }
                        _ => winit::ControlFlow::Continue,
                    },
                    _ => winit::ControlFlow::Continue,
                },
                _ => winit::ControlFlow::Continue,
            }
        });
        if done {
            return;
        }
    }

}
