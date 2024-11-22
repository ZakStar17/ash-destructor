mod impl_device_destroyable;

pub use impl_device_destroyable::ImplDeviceDestroyable;

pub use ash::vk;

pub fn create_dummy_device() -> ash::Device {
    // todo: find a way to initialize a dummy device without actually starting Vulkan

    // quite an unsafe way to do this
    let entry = unsafe { ash::Entry::load().unwrap() };
    let instance = unsafe {
        entry
            .create_instance(&vk::InstanceCreateInfo::default(), None)
            .unwrap()
    };
    let physical_device = unsafe { instance.enumerate_physical_devices().unwrap()[0] };
    let device = unsafe {
        instance
            .create_device(physical_device, &vk::DeviceCreateInfo::default(), None)
            .unwrap()
    };
    device
}
