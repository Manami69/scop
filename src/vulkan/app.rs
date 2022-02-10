use std::collections::HashSet;
use std::ffi::CStr;
use std::os::raw::c_void;
use vulkanalia::vk::ExtDebugUtilsExtension;
use vulkanalia::window as vk_window;
use vulkanalia::{
    loader::{LibloadingLoader, LIBRARY},
    prelude::v1_0::*,
    Entry, Instance,
};
use winit::window::Window;

/// Validation layers
///
/// Only on when compiled in debug mod
/// https://kylemayes.github.io/vulkanalia/setup/validation_layers.html
const VALIDATION_ENABLED: bool = cfg!(debug_assertions);

const VALIDATION_LAYER: vk::ExtensionName =
    vk::ExtensionName::from_bytes(b"VK_LAYER_KHRONOS_validation");

#[derive(Clone, Debug)]
pub struct App {
    /// INSTANCE
    ///
    ///  The instance is the connection between your application and the Vulkan library
    instance: Instance,
    entry: Entry,
}

impl App {
    pub unsafe fn init(
        window: &Window,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let loader = vulkanalia::loader::LibloadingLoader::new(LIBRARY)?;
        let entry = Entry::new(loader)?;
        let instance = App::create_instance(window, &entry)?;
        Ok(Self { entry, instance })
    }

    pub unsafe fn main_loop(&mut self, window: &Window) -> Result<(), ()> {
        Ok(())
    }

    pub unsafe fn cleanup(&mut self) {
        self.instance.destroy_instance(None);
    }
}

impl App {
    /// CREATE INSTANCE
    ///
    /// https://kylemayes.github.io/vulkanalia/setup/instance.html
    unsafe fn create_instance(
        window: &Window,
        entry: &Entry,
    ) -> Result<Instance, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let application_info = vk::ApplicationInfo::builder()
            .application_name(b"Scop\0")
            .application_version(vk::make_version(1, 0, 0))
            .engine_name(b"No Engine\0")
            .engine_version(vk::make_version(1, 0, 0))
            .api_version(vk::make_version(1, 0, 0));

        // DEBUG MOD VALIDATION LAYER

        let available_layers = entry
            .enumerate_instance_layer_properties()?
            .iter()
            .map(|l| l.layer_name)
            .collect::<HashSet<_>>();

        if VALIDATION_ENABLED && !available_layers.contains(&VALIDATION_LAYER) {
            return Err("Validation layer requested but not supported.".into());
        }

        let layers = if VALIDATION_ENABLED {
            vec![VALIDATION_LAYER.as_ptr()]
        } else {
            Vec::new()
        };

        //
        let mut extensions = vk_window::get_required_instance_extensions(window)
            .iter()
            .map(|e| e.as_ptr())
            .collect::<Vec<_>>();
        if VALIDATION_ENABLED {
            extensions.push(vk::EXT_DEBUG_UTILS_EXTENSION.name.as_ptr());
        }
        let info = vk::InstanceCreateInfo::builder()
            .application_info(&application_info)
            .enabled_layer_names(&layers)
            .enabled_extension_names(&extensions);

        Ok(entry.create_instance(&info, None)?)
    }
}
