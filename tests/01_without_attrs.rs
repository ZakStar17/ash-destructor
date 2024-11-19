use ash::vk;
use ash_destructor::DeviceDestroyable;

struct ImplDeviceDestroyable {}

impl DeviceDestroyable for ImplDeviceDestroyable {
    unsafe fn destroy_self(&self, _device: &ash::Device) {
        
    }
}

#[derive(DeviceDestroyable)]
struct Named {
    a: ImplDeviceDestroyable,
    b: ImplDeviceDestroyable
}

#[derive(DeviceDestroyable)]
struct Unnamed(ImplDeviceDestroyable, ImplDeviceDestroyable);

#[derive(DeviceDestroyable)]
struct Empty;


fn main() {
    // possible the most unsafe way to initialize a device
    let entry = unsafe {ash::Entry::load().unwrap() };
    let instance = unsafe { entry.create_instance(&vk::InstanceCreateInfo::default(), None).unwrap() };
    let physical_device = unsafe { instance.enumerate_physical_devices().unwrap()[0]};
    let device = unsafe { instance.create_device(physical_device, &vk::DeviceCreateInfo::default(), None).unwrap()};

    let named = Named {
        a: ImplDeviceDestroyable {},
        b: ImplDeviceDestroyable {}
    };
    let unnamed = Unnamed(ImplDeviceDestroyable {}, ImplDeviceDestroyable {});
    let empty = Empty {};
    unsafe {
        named.destroy_self(&device);
        unnamed.destroy_self(&device);
        empty.destroy_self(&device);
    }
}
