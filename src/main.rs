use ash::{vk, Entry};
use std::ffi::CString;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use ash::extensions::ext::DebugUtils;
use ash::extensions::khr::Surface;
use ash::extensions::ext::MetalSurface;

fn main() {
    // Create event loop and window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // Initialize Vulkan
    let entry = unsafe { Entry::load().unwrap() };
    let app_name = CString::new("Vulkan Renderer").unwrap();
    let engine_name = CString::new("No Engine").unwrap();

    let app_info = vk::ApplicationInfo {
        p_application_name: app_name.as_ptr(),
        s_type: vk::StructureType::APPLICATION_INFO,
        p_next: std::ptr::null(),
        application_version: vk::make_api_version(0, 1, 0, 0),
        p_engine_name: engine_name.as_ptr(),
        engine_version: vk::make_api_version(0, 1, 0, 0),
        api_version: vk::API_VERSION_1_0,
    };

    let extension_names = vec![
        Surface::name().as_ptr(),
        MetalSurface::name().as_ptr(),
    ];

    let create_info = vk::InstanceCreateInfo {
        s_type: vk::StructureType::INSTANCE_CREATE_INFO,
        p_next: std::ptr::null(),
        flags: vk::InstanceCreateFlags::empty(),
        p_application_info: &app_info,
        pp_enabled_extension_names: extension_names.as_ptr(),
        enabled_extension_count: extension_names.len() as u32,
        pp_enabled_layer_names: std::ptr::null(),
        enabled_layer_count: 0,
    };

    let instance = unsafe {
        match entry.create_instance(&create_info, None) {
            Ok(instance) => instance,
            Err(err) => {
                eprintln!("Instance creation error: {:?}", err);
                return;
            }
        }
    };

    #[cfg(target_os = "macos")]
    let _surface = {
        use winit::platform::macos::WindowExtMacOS;
        let create_info = vk::MetalSurfaceCreateInfoEXT {
            s_type: vk::StructureType::METAL_SURFACE_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            flags: vk::MetalSurfaceCreateFlagsEXT::empty(),
            p_layer: window.ns_view() as *const std::ffi::c_void,
        };
        unsafe {
            MetalSurface::new(&entry, &instance)
                .create_metal_surface(&create_info, None)
                .unwrap()
        }
    };

    // Event loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            _ => (),
        }
    });

    // Clean up
    // Note: This code is unreachable, needs proper event loop handling.
    // unsafe {
    //     instance.destroy_instance(None);
    // }
}
