use rand::prelude::*;
#[macro_use]
extern crate vulkano;
use std::sync::Arc;
use std::collections::HashSet;

use vulkano::device::{Device, DeviceExtensions, Queue, Features};

use vulkano::format::Format;
use vulkano::image::{ImageUsage, swapchain::SwapchainImage};
use vulkano::sync::{self, SharingMode, GpuFuture};



use vulkano::buffer::{
    TypedBufferAccess,
    immutable::ImmutableBuffer,
    BufferUsage,
    BufferAccess
};


mod vulkan_application;
use vulkan_application::VulkanApplication;

#[derive(Default, Copy, Clone)]
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
        Vertex::new([rand::thread_rng().gen(), -0.5], [1.0, 0.0, 0.0]),
        Vertex::new([0.5, -0.5], [0.0, 1.0, 0.0]),
        Vertex::new([rand::thread_rng().gen(), 0.5], [0.0, 0.0, 1.0]),
        Vertex::new([-0.5, rand::thread_rng().gen()], [1.0, 1.0, 1.0]),
        Vertex::new([rand::thread_rng().gen(), rand::thread_rng().gen()], [1.0, 1.0, 1.0])
    ].to_vec()
}

pub fn indices() -> Vec<u16> {
    [0, 1, 2, 2, 3, 0, 0, 5, 3].to_vec()
}

fn create_index_buffer(graphics_queue: &Arc<Queue>) -> Arc<TypedBufferAccess<Content=[u16]> + Send + Sync> {
    let (buffer, future) = ImmutableBuffer::from_iter(
        indices().iter().cloned(), BufferUsage::index_buffer(),
        graphics_queue.clone())
        .unwrap();
    future.flush().unwrap();
    buffer
}

fn create_vertex_buffer(graphics_queue: &Arc<Queue>) -> Arc<BufferAccess + Send + Sync> {
    let (buffer, future) = ImmutableBuffer::from_iter(
        vertices().iter().cloned(), BufferUsage::vertex_buffer(),
        graphics_queue.clone())
        .unwrap();
    future.flush().unwrap();
    buffer
}

fn main() {
    let mut app = VulkanApplication::initialize();
    app.main_loop(&vertices(), &indices());

}
