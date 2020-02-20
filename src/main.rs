use vulkan_tutorial::{
    utility::window::{ProgramProc}
};

mod vulkan_application;
use vulkan_application::VulkanApplication;

fn main() {
    let program_proc = ProgramProc::new();
    let vulkan_app = VulkanApplication::new(&program_proc.event_loop);

    program_proc.main_loop(vulkan_app);
}
