
use vulkano::swapchain::Surface;
use std::sync::Arc;
use vulkano_win::VkSurfaceBuild;

use winit::{EventsLoop, WindowBuilder, Window, dpi::LogicalSize};
use vulkano::instance::{
    Instance,
    InstanceExtensions,
    ApplicationInfo,
    Version,
    layers_list,
};
use winit::{WindowEvent, ElementState, VirtualKeyCode, KeyboardInput};

use rand::prelude::*;
#[macro_use]
extern crate vulkano;


#[cfg(all(debug_assertions))]
const ENABLE_VALIDATION_LAYERS: bool = true;
#[cfg(not(debug_assertions))]
const ENABLE_VALIDATION_LAYERS: bool = false;

const VALIDATION_LAYERS: &[&str] =  &[
    "VK_LAYER_LUNARG_standard_validation"
];

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

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

pub fn create_instance() -> Arc<Instance> {
        if ENABLE_VALIDATION_LAYERS && !check_validation_layer_support() {
            println!("Validation layers requested, but not available!")
        }

        let supported_extensions = InstanceExtensions::supported_by_core()
            .expect("failed to retrieve supported extensions");
        println!("Supported extensions: {:?}", supported_extensions);

        let app_info = ApplicationInfo {
            application_name: Some("Hello Triangle".into()),
            application_version: Some(Version { major: 1, minor: 0, patch: 0 }),
            engine_name: Some("No Engine".into()),
            engine_version: Some(Version { major: 1, minor: 0, patch: 0 }),
        };

        let required_extensions = get_required_extensions();

        if ENABLE_VALIDATION_LAYERS && check_validation_layer_support() {
            Instance::new(Some(&app_info), &required_extensions, VALIDATION_LAYERS.iter().cloned())
                .expect("failed to create Vulkan instance")
        } else {
            Instance::new(Some(&app_info), &required_extensions, None)
                .expect("failed to create Vulkan instance")
        }

    }

pub fn create_surface(instance: &Arc<Instance>) -> (EventsLoop, Arc<Surface<Window>>) {
    let events_loop = EventsLoop::new();
    let surface = WindowBuilder::new()
        .with_title("Vulkan")
        .with_dimensions(LogicalSize::new(f64::from(WIDTH), f64::from(HEIGHT)))
        .build_vk_surface(&events_loop, instance.clone())
        .expect("failed to create window surface!");
        (events_loop, surface)
}

 fn check_validation_layer_support() -> bool {
        let layers: Vec<_> = layers_list().unwrap().map(|l| l.name().to_owned()).collect();
        VALIDATION_LAYERS.iter()
            .all(|layer_name| layers.contains(&layer_name.to_string()))
    }

    fn get_required_extensions() -> InstanceExtensions {
        let mut extensions = vulkano_win::required_extensions();
        if ENABLE_VALIDATION_LAYERS {
            // TODO!: this should be ext_debug_utils (_report is deprecated), but that doesn't exist yet in vulkano
            extensions.ext_debug_report = true;
        }

        extensions
    }

fn main() {
    let mut instance = create_instance();
    let (mut events_loop, mut surface) = create_surface(&instance);
    let mut app = VulkanApplication::initialize(& mut instance, & mut surface);
    let mut my_vertices = vertices();
    let indices = indices();
    loop {
        let mut done = false;
        events_loop.run_forever(|event| {
            app.draw_frame(&my_vertices, &indices);

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
                            my_vertices = vertices();
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
