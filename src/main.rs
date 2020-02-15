use vulkan_tutorial::{
    utility,
    utility::constants::WINDOW_TITLE,
    utility::constants::WINDOW_WIDTH,
    utility::constants::WINDOW_HEIGHT,
};

use winit::event_loop::{EventLoop};


mod vulkan_application;
use vulkan_application::VulkanApplication;

fn main() {
    let event_loop = EventLoop::new();
    let window = utility::window::init_window(&event_loop, WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT);

    let vulkan_app = VulkanApplication::new();
    vulkan_app.main_loop(event_loop, window);
}
