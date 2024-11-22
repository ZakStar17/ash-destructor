use ash_destructor::DeviceDestroyable;

struct ImplDeviceDestroyable {}

impl DeviceDestroyable for ImplDeviceDestroyable {
    unsafe fn destroy_self(&self, _device: &ash::Device) {}
}

#[derive(DeviceDestroyable)]
struct Named {
    a: ImplDeviceDestroyable,
    #[skip]
    #[skip]
    b: String,
}

#[derive(DeviceDestroyable)]
struct Unnamed(
    ImplDeviceDestroyable,
    #[skip]
    #[skip]
    String,
);

fn main() {}
