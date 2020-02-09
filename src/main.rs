use winit::event_loop::{EventLoop};

mod constants;
mod vulkan_application;
use vulkan_application::VulkanApplication;

fn main() {

    let event_loop = EventLoop::new();
    let window = VulkanApplication::init_window(&event_loop);

    let vulkan_app = VulkanApplication::new();
    vulkan_app.main_loop(event_loop, window);
}
