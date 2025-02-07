/*
 * Core Rendering Engine for Life Browser
 * Implements GPU-accelerated rendering using Vulkan.
 * Author: sujit9331
 * License: Apache License 2.0
 */

use ash::version::{EntryV1_0, InstanceV1_0};
use ash::vk;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub struct VulkanApp {
    entry: ash::Entry,
    instance: ash::Instance,
    surface_loader: ash::extensions::khr::Surface,
    surface: vk::SurfaceKHR,
    physical_device: vk::PhysicalDevice,
    device: ash::Device,
    queue: vk::Queue,
    queue_family_index: u32,
    viewports: Vec<Viewport>, // New field for managing multiple viewports
}

pub struct Viewport {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl VulkanApp {
    pub fn new(window: &winit::window::Window) -> VulkanApp {
        // Load Vulkan entry
        let entry = ash::Entry::linked();

        // Create Vulkan instance
        let app_name = std::ffi::CString::new("Life Browser").unwrap();
        let engine_name = std::ffi::CString::new("Life Engine").unwrap();
        let app_info = vk::ApplicationInfo {
            s_type: vk::StructureType::APPLICATION_INFO,
            p_next: std::ptr::null(),
            p_application_name: app_name.as_ptr(),
            application_version: vk::make_version(1, 0, 0),
            p_engine_name: engine_name.as_ptr(),
            engine_version: vk::make_version(1, 0, 0),
            api_version: vk::make_version(1, 2, 0),
        };

        let instance_create_info = vk::InstanceCreateInfo {
            s_type: vk::StructureType::INSTANCE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: vk::InstanceCreateFlags::empty(),
            p_application_info: &app_info,
            enabled_layer_count: 0,
            pp_enabled_layer_names: std::ptr::null(),
            enabled_extension_count: 0,
            pp_enabled_extension_names: std::ptr::null(),
        };

        let instance = unsafe {
            entry
                .create_instance(&instance_create_info, None)
                .expect("Failed to create Vulkan instance")
        };

        // Create surface for rendering
        let surface = unsafe {
            ash_window::create_surface(&entry, &instance, window, None)
                .expect("Failed to create Vulkan surface")
        };
        let surface_loader = ash::extensions::khr::Surface::new(&entry, &instance);

        // Select physical device (GPU)
        let physical_device = unsafe {
            instance
                .enumerate_physical_devices()
                .expect("Failed to enumerate physical devices")[0]
        };

        // Find queue family index
        let queue_family_index = unsafe {
            instance
                .get_physical_device_queue_family_properties(physical_device)
                .iter()
                .enumerate()
                .position(|(index, ref info)| {
                    info.queue_flags.contains(vk::QueueFlags::GRAPHICS)
                        && surface_loader
                            .get_physical_device_surface_support(
                                physical_device,
                                index as u32,
                                surface,
                            )
                            .unwrap()
                })
                .expect("Failed to find a suitable queue family") as u32
        };

        // Create logical device and queue
        let device_queue_create_info = vk::DeviceQueueCreateInfo {
            s_type: vk::StructureType::DEVICE_QUEUE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: vk::DeviceQueueCreateFlags::empty(),
            queue_family_index,
            queue_count: 1,
            p_queue_priorities: &[1.0],
        };

        let device_create_info = vk::DeviceCreateInfo {
            s_type: vk::StructureType::DEVICE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: vk::DeviceCreateFlags::empty(),
            queue_create_info_count: 1,
            p_queue_create_infos: &device_queue_create_info,
            enabled_layer_count: 0,
            pp_enabled_layer_names: std::ptr::null(),
            enabled_extension_count: 0,
            pp_enabled_extension_names: std::ptr::null(),
            p_enabled_features: std::ptr::null(),
        };

        let device = unsafe {
            instance
                .create_device(physical_device, &device_create_info, None)
                .expect("Failed to create logical device")
        };

        let queue = unsafe { device.get_device_queue(queue_family_index, 0) };

        VulkanApp {
            entry,
            instance,
            surface_loader,
            surface,
            physical_device,
            device,
            queue,
            queue_family_index,
            viewports: vec![
                Viewport { x: 0.0, y: 0.0, width: 0.5, height: 1.0 }, // Left viewport
                Viewport { x: 0.5, y: 0.0, width: 0.5, height: 1.0 }, // Right viewport
            ],
        }
    }

    pub fn render(&self) {
        for (i, viewport) in self.viewports.iter().enumerate() {
            println!(
                "Rendering viewport {} at position ({}, {}) with size ({}, {})",
                i, viewport.x, viewport.y, viewport.width, viewport.height
            );
            // Add Vulkan rendering logic for each viewport here
        }

        // Render floating tabs
        self.render_floating_tabs();
    }

    fn render_floating_tabs(&self) {
        println!("Rendering floating tabs...");
        // Add Vulkan rendering logic for floating tabs here
    }
}

impl Drop for VulkanApp {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_device(None);
            self.surface_loader.destroy_surface(self.surface, None);
            self.instance.destroy_instance(None);
        }
    }
}

fn main() {
    // Create a window using winit
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Life Browser")
        .build(&event_loop)
        .unwrap();

    // Initialize Vulkan application
    let vulkan_app = VulkanApp::new(&window);

    // Run the event loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::MainEventsCleared => {
                vulkan_app.render();
            }
            _ => (),
        }
    });
}