#[macro_use]
extern crate vulkano;

mod vulkan_application;
use vulkan_application::VulkanApplication;

#[derive(Default, Copy, Clone)]
struct Vertex {
    pos: [f32; 2],
    color: [f32; 3],
}

impl Vertex {
    fn new(pos: [f32; 2], color: [f32; 3]) -> Self {
        Self { pos, color }
    }
}

vulkano::impl_vertex!(Vertex, pos, color);

fn vertices() -> [Vertex; 5] {
    [
        Vertex::new([-0.5, -0.5], [1.0, 0.0, 0.0]),
        Vertex::new([0.5, -0.5], [0.0, 1.0, 0.0]),
        Vertex::new([0.5, 0.5], [0.0, 0.0, 1.0]),
        Vertex::new([-0.5, 0.5], [1.0, 1.0, 1.0]),
        Vertex::new([0.0, 0.0], [1.0, 1.0, 1.0])

    ]
}

fn indices() -> [u16; 9] {
    [0, 1, 2, 2, 3, 0, 0, 5, 3]
}

fn main() {
    let mut app = VulkanApplication::initialize();
    app.main_loop();

}
